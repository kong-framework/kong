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

## 🖇️ Links

- [Source Code](https://github.com/kwatafana/kong)
- [Rust documentation](https://kong.kwatafana.org/rust/doc/kong/index.html)

## ✨ `kong` example

``` rust
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
```

## ⭐ Features

- [x] __Logging__
  - [x] Console logging
  - [x] File logging
  
## 🗺️ `kong` Roadmap

- [ ] API `0.1.0`
- [ ] API Refinements `0.2.0`
- [ ] API Refactor, Tests and documentation `0.3.0`
- [ ] Core Security `0.4.0`
- [ ] Core Security Refinements `0.5.0`
- [ ] Pen-testing `0.6.0`
- [ ] Performance Enhancements `0.7.0`
- [ ] In depth Testing, Refactor and Fuzzing `0.8.0`
- [ ] Pen-testing, Security Audit `0.9.0`
- [ ] In depth documentation `1.0.0`
- [ ] Research `1.1.0`

## 🎫 Unlicense

Written and placed in the public domain by Jackson G. Kaindume.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

For more information, please refer to the __UNLICENSE__ file in the
repository.

---

🌐 <https://kong.kwatafana.org/> | 📧 <cy6erlion@protonmail.com>
