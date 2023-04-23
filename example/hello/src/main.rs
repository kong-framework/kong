use kong::prelude::{
    kdata::resource::{GenericResource, ResourceError},
    kroute, *,
};

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

    fn kontrol(&self, _kong: &Kong) -> Result<serde_json::Value, ResourceError> {
        let resource = GenericResource {
            message: "Hello, World".to_string(),
        };
        Ok(resource.as_json())
    }
}
