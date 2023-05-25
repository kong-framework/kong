use kong::{json, kroute, server, Kong, Kontrol, Method};

fn main() {
    // start kong router, kontrolling provided endpoint kontrollers
    kroute(vec![Box::new(HelloKontroller {
        address: "/hello".to_string(),
        method: Method::Get,
    })]);
}

/// Hello API endpoint controller
struct HelloKontroller {
    /// Endpoint address
    address: String,
    /// Endpoint HTTP method
    method: Method,
}
impl Kontrol for HelloKontroller {
    fn address(&self) -> String {
        self.address.clone()
    }

    fn method(&self) -> Method {
        self.method
    }

    fn kontrol(&self, _kong: &Kong) -> server::Response {
        let res = json!({ "message": "Hello World" });
        server::Response::json(&res).with_status_code(200)
    }
}
