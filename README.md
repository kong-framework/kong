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
use kong::prelude::{
    kdata::resource::{GenericResource, ResourceError},
    kroute, *,
};

fn main() {
    let port = Konfig::read_port();
    let address = format!("localhost:{}", port);

    println!("kong example running @ {address}");
    kroute(&address, vec![Box::new(HelloKontrolHandler)]);
}

/// Hello API endpoint controller
struct HelloKontrolHandler;
impl Kontrol for HelloKontrolHandler {
    fn address(&self) -> String {
        "/hello".to_string()
    }

    fn method(&self) -> Method {
        Method::Get
    }

    fn kontrol(&self, _kong: &Kong) -> Result<serde_json::Value, ResourceError> {
        let resource = GenericResource::json("Hello, World");
        Ok(resource)
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
