use std::io;
use std::time::Duration;
use tokio::time::Instant;
use crate::utils::join_all_with_semaphore;
use crate::outputs::{ValidHosts, ValidIpV4s, ValidIpV6s};
use crate::servers::StunServer;
use crate::stun::{StunServerTestResult, StunSocketResponse};

extern crate pretty_env_logger;
#[macro_use] extern crate log;

mod servers;
mod stun;
mod utils;
mod outputs;
mod geoip;

const CONCURRENT_SOCKETS_USED_LIMIT: usize = 64;

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    pretty_env_logger::init();

    let mut client = geoip::CachedIpGeolocationIpClient::default().await?;

    client.save().await?;

    let stun_servers = servers::get_stun_servers().await?;

    let stun_servers_count = stun_servers.len();
    info!("Loaded {} stun server hosts", stun_servers.len());

    let stun_server_test_results = stun_servers.into_iter()
        .map(|candidate| {
            async move {
                let test_result = stun::test_udp_stun_server(candidate).await;
                print_stun_server_status(&test_result);
                test_result
            }
        })
        .collect::<Vec<_>>();

    let timestamp = Instant::now();
    let stun_server_test_results = join_all_with_semaphore(stun_server_test_results.into_iter(), CONCURRENT_SOCKETS_USED_LIMIT).await;

    stun_server_test_results.iter()
        .filter(|test_result| test_result.is_healthy())
        .for_each(|test_result| {
            async {
                client.get_hostname_geoip_info(test_result.server.hostname.as_str()).await;
                test_result.socket_tests.iter().for_each(|socket| {
                    async {
                        client.get_ip_geoip_info(socket.socket.ip()).await;
                    };
                });
            };
    });

    client.save().await;

    ValidHosts::default(&stun_server_test_results).save().await?;
    ValidIpV4s::default(&stun_server_test_results).save().await?;
    ValidIpV6s::default(&stun_server_test_results).save().await?;

    write_stun_server_summary(stun_servers_count, &stun_server_test_results,timestamp.elapsed());

    Ok(())
}

fn print_stun_server_status(test_result: &StunServerTestResult) {
    if test_result.is_healthy() {
        info!("{:<25} -> Host is healthy", test_result.server.hostname);
    } else if !test_result.is_resolvable() {
        info!("{:<25} -> Host is not resolvable", test_result.server.hostname);
    } else if test_result.is_partial_timeout() {
        info!("{:<25} -> Host times out on some sockets", test_result.server.hostname);
    } else if test_result.is_timeout() {
        info!("{:<25} -> Host times out on all sockets", test_result.server.hostname);
    } else {
        info!("{:<25} -> Host behaves in an unexpected way. Run with RUST_LOG=DEBUG for more info", test_result.server.hostname);
        for socket_test in &test_result.socket_tests {
            match &socket_test.result {
                StunSocketResponse::HealthyResponse { .. } => { debug!("{:<25} -> Socket {:<21} returned a healthy response", test_result.server.hostname, socket_test.socket) }
                StunSocketResponse::InvalidMappingResponse { expected, actual, rtt } => { debug!("{:<25} -> Socket {:<21} returned an invalid mapping: expected={} actual={}", test_result.server.hostname, socket_test.socket, expected, actual) }
                StunSocketResponse::Timeout { deadline } => { debug!("{:<25} -> Socket {:<21} timed out after {:?}", test_result.server.hostname, socket_test.socket, deadline) }
                StunSocketResponse::UnexpectedError { err } => { debug!("{:<25} -> Socket {:<21} returned an unexpected error: {}", test_result.server.hostname, socket_test.socket, err) }
            }
        }
    }
}

fn write_stun_server_summary(candidate_hosts_count: usize, results: &Vec<StunServerTestResult>, time_taken: Duration) {
    let mut healthy = 0;
    let mut dns_unresolved = 0;
    let mut partial_timeout = 0;
    let mut timeout = 0;
    let mut unexpected_err = 0;
    results.iter().for_each(|server_test_result| {
        if server_test_result.is_healthy() {
            healthy += 1;
        } else if !server_test_result.is_resolvable() {
            dns_unresolved += 1;
        } else if server_test_result.is_partial_timeout() {
            partial_timeout += 1;
        } else if server_test_result.is_timeout() {
            timeout += 1;
        } else {
            unexpected_err += 1;
        }
    });
    info!(
        "Statistics -> Tested={}, Healthy={}, DNS failure={}, partial Timeout={}, Timeout={}, Unexpected err={}. Finished in {:?}",
        candidate_hosts_count, healthy, dns_unresolved, partial_timeout, timeout, unexpected_err, time_taken
    );

    if healthy == 0 {
        warn!("No healthy hosts found! Are you behind NAT?")
    }
}
