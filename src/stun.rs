use std::net::{SocketAddr};
use std::time::{Duration, Instant};
use crate::utils::join_all_with_semaphore;
use crate::StunServer;

pub(crate) struct StunServerTestResult {
    pub(crate) server: StunServer,
    pub(crate) socket_tests: Vec<StunSocketTestResult>,
}

impl StunServerTestResult {
    pub(crate) fn is_resolvable(&self) -> bool {
        return self.socket_tests.len() > 0;
    }

    pub(crate) fn is_healthy(&self) -> bool {
        return self.is_resolvable() && self.socket_tests.iter()
            .all(StunSocketTestResult::is_ok);
    }

    pub(crate) fn is_partial_timeout(&self) -> bool {
        return self.is_resolvable() && self.socket_tests.iter()
            .all(|result| match result.result {
                StunSocketResponse::HealthyResponse {..} => true,
                StunSocketResponse::Timeout {..} => true,
                _ => false,
            });
    }

    pub(crate) fn is_timeout(&self) -> bool {
        return self.is_resolvable() && self.socket_tests.iter()
            .all(|result| if let StunSocketResponse::Timeout {..} = result.result { true } else { false });
    }
}

pub(crate) struct StunSocketTestResult {
    pub(crate) socket: SocketAddr,
    pub(crate) result: StunSocketResponse
}

impl StunSocketTestResult {
    pub(crate) fn is_ok(&self) -> bool {
        self.result.is_ok()
    }
}

pub(crate) enum StunSocketResponse {
    HealthyResponse { rtt: Duration },
    InvalidMappingResponse { expected: SocketAddr, actual: SocketAddr, rtt: Duration },
    Timeout { deadline: Duration },
    UnexpectedError { err: String }
}

impl StunSocketResponse {
    fn is_ok(&self) -> bool {
        match &self {
            StunSocketResponse::HealthyResponse { .. } => true,
            _ => false
        }
    }
}

pub(crate) async fn test_udp_stun_server(
    server: StunServer
) -> StunServerTestResult {
    let socket_addrs = tokio::net::lookup_host(format!("{}:{}", server.hostname, server.port)).await;

    if socket_addrs.is_err() {
        println!("{} -> DNS Failure {:?}", server.hostname, socket_addrs.err().unwrap());
        return StunServerTestResult {
            server,
            socket_tests: vec![],
        }
    }

    let results = socket_addrs.unwrap()
        .map(|addr| addr.ip())
        .map(|addr| {
            let port = server.port;
            async move {
                let stun_socket = SocketAddr::new(addr, port);
                let res = test_socket_addr(stun_socket).await;
                res
            }
        })
        .collect::<Vec<_>>();

    let results = join_all_with_semaphore(results.into_iter(), 1).await;

    StunServerTestResult {
        server,
        socket_tests: results
    }
}

async fn test_socket_addr(
    socket_addr: SocketAddr
) -> StunSocketTestResult {
    let local_socket = tokio::net::UdpSocket::bind(
        match socket_addr {
            SocketAddr::V4(..) => { "0.0.0.0:0" }
            SocketAddr::V6(..) => { "[::]:0" }
        }
    ).await.unwrap();

    let mut client = stunclient::StunClient::new(socket_addr);
    let deadline = Duration::from_secs(1);
    client.set_timeout(deadline);

    let start = Instant::now();

    let result = client.query_external_address_async(&local_socket).await;

    let request_duration = Instant::now() - start;

    return match result {
        Ok(return_addr) => if return_addr.port() == local_socket.local_addr().unwrap().port() {
            StunSocketTestResult {
                socket: socket_addr,
                result: StunSocketResponse::HealthyResponse { rtt: request_duration },
            }
        } else {
            StunSocketTestResult {
                socket: socket_addr,
                result: StunSocketResponse::InvalidMappingResponse { expected: local_socket.local_addr().unwrap(), actual: return_addr, rtt: request_duration },
            }
        },
        Err(err) => {
            if err.to_string() == "Timed out waiting for STUN server reply" {
                StunSocketTestResult {
                    socket: socket_addr,
                    result: StunSocketResponse::Timeout { deadline },
                }
            } else {
                StunSocketTestResult {
                    socket: socket_addr,
                    result: StunSocketResponse::UnexpectedError { err: err.to_string() },
                }
            }
        },
    }
}
