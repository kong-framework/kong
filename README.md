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
    kdata::{
        inputs::NoInput,
        resource::{GenericResource, ResourceError},
    },
    *,
};
use std::sync::Mutex;

fn main() -> Result<(), KError> {
    let kong: Mutex<Kong<_, _>> = Mutex::new(Kong::new(vec![HelloKontroller::kontroller()]));
    kong.lock().unwrap().database.connect()?;

    let port = Konfig::read_port();
    let address = format!("localhost:{}", port);

    println!("kong example running @ {address}");
    server::start_server(address, move |request| {
        let mut kong = kong.lock().unwrap();
        Kroute::kroute(&mut kong, request)
    });
}

/// Hello API endpoint controller
pub struct HelloKontroller;

/// Endpoint kontrolers should impl the KontrolHandle trait,
/// `kong` does not enforce this, you can use standalone functions
/// for `get_input`, `handle` and `kontroller` but the `KontrolHandle`
/// trait is more ergonomic and consistent.
/// `NoInput` is the input expected, it is used for endpoint that
/// don't expect user input. If an endpoint does expect user input
/// any type can be used the impl the `UserInput` trait.
/// `GenericResource` is the resource that the endpoint accesses,
/// `GenericResource` is a placeholder for endpoints that don't
/// access any `Resource`, any type that impl Resource can be used
impl KontrolHandle<NoInput, GenericResource> for HelloKontroller {
    /// Hello endpoint handler
    fn handle(
        _kong: &mut Kong<NoInput, GenericResource>,
        _input: Option<NoInput>,
        _kpassport: Option<kpassport::Kpassport>,
    ) -> Result<GenericResource, ResourceError> {
        Ok(GenericResource {
            message: "Hello, World!".to_string(),
        })
    }

    /// Create an new account, can be used by users of kong to get a HelloKontroller Kontroller
    fn kontroller<'a>() -> Kontroller<'a, NoInput, GenericResource> {
        // Endpoint handling
        let hello: Kontrol<NoInput, GenericResource> = Kontrol {
            // The function used to extract user input, here it is
            // None because we don't expect any input for this kontroller
            get_input: None,
            // The function used to validate user input, here it is
            // None because we don't expect any input for this kontroller.
            // This can ofcourse be part of `get_input` above, but
            // having it explicitly like this ensures that validation is
            // not hidden or forgotten
            validate: None,
            // The actual bussiness logic, or endpoint handler
            kontrol: HelloKontroller::kontrol,
        };

        Kontroller {
            // The endpoint's address
            address: "/hello",
            // The method that the endpoint supports
            method: Method::Get,
            // Endpoint handler
            kontrol: hello,
        }
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
