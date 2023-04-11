use kong::prelude::*;
use std::sync::Mutex;

/// Handle request
fn welcome(
    _kong: &mut Kong<kdata::inputs::NoInput>,
    _input: Option<kdata::inputs::NoInput>,
) -> server::Response {
    server::Response::text("Welcome!")
}

/// Handle request
fn say_hello(
    _kong: &mut Kong<kdata::inputs::NoInput>,
    _input: Option<kdata::inputs::NoInput>,
) -> server::Response {
    server::Response::text("Hello; world!")
}

fn say_bye(
    _kong: &mut Kong<kdata::inputs::NoInput>,
    _input: Option<kdata::inputs::NoInput>,
) -> server::Response {
    server::Response::text("bye bye!")
}

fn main() -> Result<(), KError> {
    let welcome_kontrol: Kontrol<kdata::inputs::NoInput> = Kontrol {
        get_input: None,
        validate: None,
        kontrol: welcome,
    };

    let hello_kontrol: Kontrol<kdata::inputs::NoInput> = Kontrol {
        get_input: None,
        validate: None,
        kontrol: say_hello,
    };

    let bye_kontrol: Kontrol<kdata::inputs::NoInput> = Kontrol {
        get_input: None,
        validate: None,
        kontrol: say_bye,
    };

    let welcome_kontroller = Kontroller {
        address: "/",
        kontrol: welcome_kontrol,
        method: Method::Get,
    };

    let hello_kontroller = Kontroller {
        address: "/hello",
        kontrol: hello_kontrol,
        method: Method::Get,
    };

    let bye_kontroller = Kontroller {
        address: "/bye",
        kontrol: bye_kontrol,
        method: Method::Get,
    };

    let kong: Mutex<Kong<_>> = Mutex::new(Kong::new(vec![
        welcome_kontroller,
        hello_kontroller,
        bye_kontroller,
    ]));
    kong.lock().unwrap().database.connect()?;

    let port = Konfig::read_port();
    let address = format!("localhost:{}", port);

    println!("kong example running @ {address}");
    server::start_server(address, move |request| {
        let mut kong = kong.lock().unwrap();
        Kroute::kroute(&mut kong, request)
    });
}
