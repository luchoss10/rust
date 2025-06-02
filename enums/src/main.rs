enum IpAddrKind1 {
    V4,
    V6,
}

fn route_1(_ip_kind: IpAddrKind1) {}

enum IpAddr {
    V4(String),
    V6(String),
}

enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body woulde be defined here
    }
}

fn main() {
    let four = IpAddrKind1::V4;
    let six = IpAddrKind1::V6;

    route_1(four);
    route_1(six);

    let _home = IpAddr::V4(String::from("127.0.0.1"));
    let _loopback = IpAddr::V6(String::from("::1"));

    let _home = IpAddr2::V4(127, 0, 0, 1);
    let _loopback = IpAddr2::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let _x: i8 = 5;
    let _y: Option<i8> = Some(5);

    //let sum = x + y; // error: mismatched types
}
