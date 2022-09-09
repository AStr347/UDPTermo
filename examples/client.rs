use std::net::UdpSocket;

fn main() {
    let udp = UdpSocket::bind("127.0.0.1:55231").expect("can't bind soket");
    udp.connect("127.0.0.1:55441").unwrap();
    let id : u64 = 0xabcdef;
    let msg: &[u8] = &id.to_be_bytes();
    udp.send(msg).unwrap();
    let mut buf :[u8 ; 4] = [0; 4];
    loop {
        let res = udp.recv(&mut buf);
        match res {
            Ok(_) => {
                let temp : f32 = f32::from_be_bytes(buf);
                println!("msg: {}", temp);
            },
            Err(e) => println!("error: {}", e),
        }
    }
}