use udptermo::UdpTermo;


#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    let termo = UdpTermo{ id: 0xabcdef, temp: 0.0};
    termo.serv_runner("127.0.0.1:55441").await;
}