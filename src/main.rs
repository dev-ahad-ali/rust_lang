enum IpAddrKind {
    V4,
    V6,
}
enum IpAddrString {
    V4(String),
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
        println!("Message enum method");
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("120.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    let m = Message::Write(String::from("Hello Message"));
    m.call();
}

fn route(ip_kind: IpAddrKind) {}
