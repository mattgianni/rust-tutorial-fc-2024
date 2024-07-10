#[allow(dead_code)]

fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // fn route(ip_kind: IpAddrKind) {}

    // route(IpAddrKind::V4);
    // route(IpAddrKind::V6);

    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // enum IpAddr {
    //     V4(String),
    //     V6(String),
    // }

    println!("home = {home:?}");
    println!("loopback = {loopback:?}");

    // struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String
    // }

    // let home: IpAddr = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // }

    // let loopback: IpAddr = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // }
}
