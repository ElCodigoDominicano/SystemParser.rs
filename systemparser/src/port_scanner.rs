use std::net::{
    IpAddr,Ipv4Addr, SocketAddr, TcpStream};
use std::time::Instant;

pub fn timer() {
    let now = Instant::now();
    let elapsed_time = now.elapsed();
    // func();
    println!("Func Took {:?}", elapsed_time);
}
pub fn port_scanner() -> Vec<String> {
    let mut open_ports = Vec::new();
    let now = Instant::now();

    for port in 0..65535 {
        let mut socket = SocketAddr::new(
            IpAddr::V4(
                Ipv4Addr::new(127, 0, 0, 1)), port);
        let sock = TcpStream::connect(&socket);

        if let Ok(stream) = TcpStream::connect(&socket) {
            open_ports.push(socket.port().to_string());
        }
    }
    open_ports
}
