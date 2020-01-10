pub fn ipadd() {
    let home = IPAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IPAddrKind::V6(String::from("::1"));
}


enum IPAddrKind { // Type
    V4(String),
    V6(String),
}

