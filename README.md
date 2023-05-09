``` text
                              )
                           ( /(          (  (
                            )\())(   (    )\))(
                           ((_)\ )\  )\ )((_))\
                           | |(_|(_)_(_/( (()(_)
                           | / / _ \ ' \)) _` |
                           |_\_\___/_||_|\__, |
                         secure web node |___/ v0.1.0
```

[documentation](https://kong.kwatafana.org/rust/doc/kong/index.html)

``` rust
use kong::{json, kroute, server, Konfig, Kong, Kontrol, Method};

fn main() {
    // Read port from configuration file
    let port = Konfig::read_port();
    // Setup server address
    let address = format!("localhost:{}", port);

    println!("kong example running @ {address}");

    // start kong router, kontrolling provided
    // provided endpoint kontrollers
    kroute(
        &address,
        vec![Box::new(HelloKontroller {
            address: "/hello".to_string(),
            method: Method::Get,
        })],
    );
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
```

## Unlicense

Written and placed in the public domain by Jackson G. Kaindume.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

For more information, please refer to the __UNLICENSE__ file in the
repository.

---

<https://kong.kwatafana.org/>
