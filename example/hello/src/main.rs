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
