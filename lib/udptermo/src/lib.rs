use std::{net::SocketAddr, time::Duration};

use tokio::{
    self,
    net::UdpSocket, time::sleep
};

use std::sync::{Arc, Mutex};
use rand::Rng;

#[derive(Clone, Copy)]
pub struct UdpTermo {
    pub id : u64,
    pub temp : f32,
}

impl UdpTermo {
    pub async fn serv_runner(self, addr: &'static str){
        let udp = UdpSocket::bind(addr).await.expect("can't bin soket addr");
        self.run(udp).await;
    }

    async fn routine(termo : Arc<Mutex<UdpTermo>>, addr: SocketAddr, udp : Arc<UdpSocket>){
        loop {
            let temp : f32;
            {
                let t = termo.lock().unwrap();
                temp = t.temp;
            }
            let msg: &[u8] = &temp.to_be_bytes();
            let res = udp.send_to(msg, addr).await;
            match res {
                Ok(_) => println!("{} sended to {}", temp, addr),
                Err(e) => println!("{}", e),
            }
            sleep(Duration::from_secs(3)).await;
        }
    }

    async fn connected_routine(termo : Arc<Mutex<UdpTermo>>, udp : Arc<UdpSocket>){
        let id: u64;
        {
            let t = termo.lock().unwrap();
            id = t.id;
        }
        loop {
            let termo = termo.clone();
            let udp = udp.clone();
            let mut buf = [0; 8];
            let res = udp.recv_from(&mut buf).await;
            if let Ok((_, addr)) = res {
                let sid = u64::from_be_bytes(buf);
                if sid == id {
                    println!("connected {}", addr);
                    let mtermo = termo.clone();
                    tokio::spawn(
                        async move {
                            let udp = udp.clone();
                            let addr = addr.clone();
                            Self::routine(mtermo, addr, udp).await;
                        }
                    );
                }
            }
            sleep(Duration::from_secs(3)).await;
            {
                let mut t = termo.lock().unwrap();
                t.temp = rand::thread_rng().gen_range(-10.0..35.0);
            }
        }
    }

    async fn run(self, udp : UdpSocket) {
        let me = Arc::new(Mutex::new(self));
        let audp = Arc::new(udp);
        Self::connected_routine(me, audp).await;
    }
}
