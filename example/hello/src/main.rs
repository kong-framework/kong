use kong::{json, kroute, server, Konfig, Kong, Kontrol, Method};

fn main() {
    let port = Konfig::read_port();
    let address = format!("localhost:{}", port);

    println!("kong example running @ {address}");
    kroute(&address, vec![Box::new(HelloKontroller)]);
}

/// Hello API endpoint controller
struct HelloKontroller;
impl Kontrol for HelloKontroller {
    fn address(&self) -> String {
        "/hello".to_string()
    }

    fn method(&self) -> Method {
        Method::Get
    }

    fn kontrol(&self, _kong: &Kong) -> server::Response {
        let res = json!({ "message": "Hello World" });
        server::Response::json(&res).with_status_code(200)
    }
}
