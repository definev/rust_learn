enum IpAddrKind {
    v4(String),           // We can assign any data type with enum
    v6((u8, u8, u8, u8)), // Type with enum can be different per enum
}

fn main() {
    let v4 = IpAddrKind::v4;
    let v6 = IpAddrKind::v6;

    let localhost_v4 = IpAddrKind::v4(String::from("127.0.0.1"));
    let localhost_v6 = IpAddrKind::v6((127, 0, 0, 1));
}