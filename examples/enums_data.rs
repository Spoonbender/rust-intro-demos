enum Endpoint {
    Hostname(String),
    IPv4(u8,u8,u8,u8)
}

fn main() {
    let ep1 = Endpoint::Hostname("www.google.com".to_owned());
    connect(ep1);

    let ep2 = Endpoint::IPv4(10,0,0,138);
    connect(ep2);
}

fn connect(endpoint: Endpoint) {
    match endpoint {
        Endpoint::Hostname(hostname) => {
            println!("Connecting to: {}", hostname);
            // ...
        },
        Endpoint::IPv4(o1, o2, o3, o4) => {
             println!("Connecting to {}.{}.{}.{}", o1, o2, o3, o4);
             // ...
        }
    }
}