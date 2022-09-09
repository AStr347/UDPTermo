use std::net::{
    UdpSocket,
    SocketAddr
};
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};

use rand::Rng;

#[derive(Clone, Copy)]
pub struct UdpTermo {
    pub id : u64,
    pub temp : f32,
}

impl UdpTermo {
    pub fn serv_runner(self, addr: &'static str){
        let udp = UdpSocket::bind(addr).expect("can't bind soket");
        self.run(udp);
    }

    fn routine(termo : Arc<Mutex<UdpTermo>>, addr: SocketAddr, udp : Arc<UdpSocket>){
        loop {
            let temp : f32;
            {
                let t = termo.lock().unwrap();
                temp = t.temp;
            }
            let msg: &[u8] = &temp.to_be_bytes();
            let res = udp.send_to(msg, addr);
            match res {
                Ok(_) => println!("{} sended to {}", temp, addr),
                Err(e) => println!("{}", e),
            }
            thread::sleep(Duration::from_secs(3));
        }
    }

    fn connected_routine(termo : Arc<Mutex<UdpTermo>>, udp : Arc<UdpSocket>){
        let id: u64;
        {
            let t = termo.lock().unwrap();
            id = t.id;
        }
        loop {
            let termo = termo.clone();
            let udp = udp.clone();
            let mut buf = [0; 8];
            let res = udp.recv_from(&mut buf);
            if let Ok((_, addr)) = res {
                let sid = u64::from_be_bytes(buf);
                if sid == id {
                    println!("connected {}", addr);
                    let mtermo = termo.clone();
                    thread::spawn(
                        move|| {
                            let udp = udp.clone();
                            let addr = addr.clone();
                            Self::routine(mtermo, addr, udp);
                        }
                    );
                }
            }
            thread::sleep(Duration::from_secs(3));
            {
                let mut t = termo.lock().unwrap();
                t.temp = rand::thread_rng().gen_range(-10.0..35.0);
            }
        }
    }

    fn run(self, udp : UdpSocket) {
        let me = Arc::new(Mutex::new(self));
        let audp = Arc::new(udp);
        audp.set_read_timeout(Some(Duration::from_secs(15))).unwrap();
        Self::connected_routine(me, audp);
    }
}
