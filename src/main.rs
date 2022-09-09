use udptermo::UdpTermo;


fn main() {
    let termo = UdpTermo{ id: 0xabcdef, temp: 0.0};
    termo.serv_runner("127.0.0.1:55441");
}
