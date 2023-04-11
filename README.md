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

## Features

- [ ] Confidentiality
- [ ] Integrity
- [ ] Availability
- [ ] Database Management
- [ ] Session Management
- [ ] HTTP+JSON API
- [ ] Optional Federation
- [ ] Built in HTTP server
- [ ] Account Management


## Usage

``` rust
use kong::prelude::*;
use std::sync::Mutex;

/// Get user input from the request
fn get_input(_request: &server::Request) -> kdata::inputs::NoInput {
    // since we don't expect input we use NoInput
    kdata::inputs::NoInput
}

/// Validate user input
fn validate(_input: kdata::inputs::NoInput) -> Result<kdata::inputs::NoInput, ()> {
    // Since we don't expect input we just return Ok
    Ok(kdata::inputs::NoInput)
}

/// Handle request
fn say_hello(
    _kong: &mut Kong<kdata::inputs::NoInput>,
    _input: kdata::inputs::NoInput,
) -> server::Response {
    server::Response::text("Hello; world!")
}

fn main() -> Result<(), KError> {
    let kontrol: Kontrol<kdata::inputs::NoInput> = Kontrol {
        get_input,
        validate,
        kontrol: say_hello,
    };

    let kontroller = Kontroller {
        address: "/",
        kontrol,
        method: Method::Get,
    };

    let kong: Mutex<Kong<_>> = Mutex::new(Kong::new(vec![kontroller]));
    kong.lock().unwrap().database.connect()?;

    let port = Konfig::read_port();
    let address = format!("localhost:{}", port);

    println!("kong example running @ {address}");
    server::start_server(address, move |request| {
        let mut kong = kong.lock().unwrap();
        Kroute::kroute(&mut kong, request)
    });
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
