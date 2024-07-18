use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};

fn main() {
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let mut addr_zeroport: SocketAddr = SocketAddr::new(ip, 8888);
    addr_zeroport.set_port(0);
    let mut ports: [u16; 5] = [0; 5];
    for i in 0..5 {
        let listener = TcpListener::bind(addr_zeroport).unwrap();
        let addr = listener.local_addr().unwrap();
        ports[i] = addr.port();
    }
    println!("{:?}", ports);
}