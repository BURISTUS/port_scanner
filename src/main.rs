use clap::{App, Arg};
use std::net::{SocketAddr, ToSocketAddrs};

use port_scanner::screen::scan;


#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
        let cli_matches = App::new(clap::crate_name!())
            .version(clap::crate_version!())
            .about(clap::crate_description!())
            .arg(
                Arg::with_name("target")
                    .help("The target to scan")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::with_name("concurrency")
                    .help("Concurrency")
                    .long("concurrency")
                    .short("c")
                    .default_value("1002"),
            )
            .arg(
                Arg::with_name("verbose")
                    .help("Display detailed information")
                    .long("verbose")
                    .short("v"),
            )
            .arg(
                Arg::with_name("full")
                    .help("Scan all 65535 ports")
                    .long("full"),
            )
            .arg(
                Arg::with_name("timeout")
                    .help("Connection timeout")
                    .long("timeout")
                    .short("t")
                    .default_value("3"),
            )
            .setting(clap::AppSettings::ArgRequiredElseHelp)
            .setting(clap::AppSettings::VersionlessSubcommands)
            .get_matches();
    
        let full = cli_matches.is_present("full");
        let verbose = cli_matches.is_present("verbose");
        let concurrency = cli_matches
            .value_of("concurrency")
            .unwrap()
            .parse::<usize>()
            .unwrap_or(1002);
        let timeout = cli_matches
            .value_of("timeout")
            .unwrap()
            .parse::<u64>()
            .unwrap_or(3);
        let target = cli_matches.value_of("target").unwrap();
    
        if verbose {
            let ports = if full {
                String::from("all the 65535 ports")
            } else {
                String::from("the most common 1002 ports")
            };
            println!(
                "Scanning {} of {}. Concurrency: {:?}. Timeout: {:?}",
                &ports, target, concurrency, timeout
            );
        }
    
        let socket_addresses: Vec<SocketAddr> = format!("{}:0", target).to_socket_addrs()?.collect();
    
        if socket_addresses.is_empty() {
            return Err(anyhow::anyhow!("Socket_addresses list is empty"));
        }
    
        scan(socket_addresses[0].ip(), full, concurrency, timeout).await;
    
        Ok(())
}
