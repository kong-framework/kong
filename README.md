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
use kong::{
    kontrol::{Kontrol, Kontroller},
    kroute::Kroute,
    outsource::server,
    KError, Konfig, Kong,
};
use server::{Request, Response};
use std::sync::Mutex;

fn main(){
    let kontrollers = vec![
        Kontroller {
            address: "/",
            handle: front_page,
        },
        Kontroller {
            address: "/news/:article",
            handle: article,
        },
    ];

    let kong: Mutex<Kong> = Mutex::new(Kong::new(kontrollers));
	
	// Connect kong database
    kong.lock().unwrap().database.connect()?;

	// Read port from the cong config file (passed in as a command line argument)
    let port = Konfig::read_port();
    let address = format!("localhost:{}", port);
	
	println!("\n: starting server...");
    println!(": running @ {address}");

    server::start_server(address, move |request| {
        let mut kong = kong.lock().unwrap();
        Kroute::kroute(&mut kong, request)
    });
}

// front page
fn front_page(kong: &mut Kong, req: &Request) -> Response {
    Response::text("Yebo Yes")
}

// News article page
fn article(kong: &mut Kong, req: &Request) -> Response {
    let params = Kroute::get_url_params(&kong.router, &req.url());

    match params {
        Ok(params) => {
            if let Some(article) = params.find("article") {
                Response::text(article).with_status_code(400)
            } else {
                Response::text("could not parse url").with_status_code(400)
            }
        }
        Err(_) => Response::text("could not parse url").with_status_code(400),
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
