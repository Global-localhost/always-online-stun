use std::io;
use tokio::time::Instant;
use crate::utils::join_all_with_semaphore;
use crate::outputs::{ValidHosts, ValidIpV4s, ValidIpV6s};
use crate::servers::StunServer;
use crate::stun::{StunServerTestResult};

mod servers;
mod stun;
mod utils;
mod outputs;

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    let stun_servers = servers::get_stun_servers().await?;

    let stun_server_test_results = stun_servers.into_iter()
        .map(|candidate| {
            async move {
                let test_result = stun::test_udp_stun_server(candidate).await;
                test_result
            }
        })
        .collect::<Vec<_>>();

    let timestamp = Instant::now();
    let stun_server_test_results = join_all_with_semaphore(stun_server_test_results.into_iter(), 100).await;

    write_stun_server_summary(&stun_server_test_results);

    ValidHosts::default(&stun_server_test_results).save().await?;
    ValidIpV4s::default(&stun_server_test_results).save().await?;
    ValidIpV6s::default(&stun_server_test_results).save().await?;

    println!("Finished in {:?}", timestamp.elapsed());
    Ok(())
}

fn write_stun_server_summary(results: &Vec<StunServerTestResult>) {
    let mut all_ok = 0;
    let mut dns_unresolved = 0;
    let mut other = 0;
    results.iter().for_each(|server_test_result| {
        if server_test_result.is_healthy() {
            all_ok += 1;
        } else if !server_test_result.is_resolvable() {
            dns_unresolved += 1;
        } else {
            other += 1;
        }
    });
    println!(
        "OK {} , DNS failure {} , Other {}",
        all_ok, dns_unresolved, other
    );
}