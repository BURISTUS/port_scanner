use futures::{stream, StreamExt};
use std::{
    net::{IpAddr, SocketAddr},
    time::Duration,
};
use tokio::net::TcpStream;

use crate::ports::ports; 

pub fn get_ports(full: bool) -> Box<dyn Iterator<Item = u16>>{
    if full {
	    Box::new((1..=u16::MAX).into_iter())
    } else {
	    Box::new(ports().to_owned().into_iter())
    }
}

pub async fn scan_port(target: IpAddr, full:bool, port: u16, timeout: u64){
    let timeout = Duration::from_secs(timeout);
    let ports = stream::iter(get_ports(full));
    let socket_adress = SocketAddr::new(target.clone(), port);

    match tokio::time::timeout(timeout, TcpStream::connect(&socket_adress)).await {
        Ok(Ok(_)) => println!("port {} is full", port),
        _ => {}
    }
}

pub async fn scan(target: IpAddr, full: bool, concurrency: usize, timeout: u64){
    let ports = stream::iter(get_ports(full));

    ports.for_each_concurrent(
        concurrency, |port| {
            println!("{}", port);
            scan_port(target, full, port, timeout)
        })
        .await
}