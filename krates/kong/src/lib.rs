#![doc(html_favicon_url = "https://kwatafana.org/logo.jpeg")]
#![doc(html_logo_url = "https://kwatafana.org/logo.jpeg")]
#![warn(missing_docs, unreachable_pub, future_incompatible, rust_2018_idioms)]

pub use kdata;
pub use kerror::KError;
pub use kollection::Kollection;
pub use konfig::Konfig;
use kontrol::{Kontrol, Kontroller};
pub mod kontrol;
pub mod outsource;
use rouille::{Request, Response};
use route_recognizer::{Params, Router};

pub struct Kong {
    pub database: Kollection,
    pub config: Konfig,
    pub router: Router<for<'a, 'b> fn(&mut Kong, &'b Request) -> Response>,
}

impl Kong {
    pub fn new<'a>(kontrollers: Vec<Kontroller<'a>>) -> Self {
        let config = Konfig::read().unwrap();
        let database = Kollection::new(&config.admin_accounts_database);
        let mut router = Router::new();

        for kontroller in &kontrollers {
            router.add(kontroller.address, kontroller.handle);
        }

        Kong {
            database,
            config,
            router,
        }
    }
    /// Start up runtime
    pub fn start(&mut self) -> Result<(), kerror::KError> {
        self.database.connect()
    }
}

pub fn kroute(kong: &mut Kong, request: &rouille::Request) -> rouille::Response {
    // Handle static files
    if let Some(path) = &kong.config.static_files_path {
        let response = rouille::match_assets(request, &path);
        if response.is_success() {
            return response;
        }
    }

    let router = kong.router.clone();
    let m = router.recognize(&request.url());

    match m {
        Ok(mut mtch) => (mtch.handler_mut())(kong, request),
        Err(_) => rouille::Response::html("404 error").with_status_code(404),
    }
}

pub fn get_url_params(
    router: &Router<for<'a, 'b> fn(&mut Kong, &'b Request) -> Response>,
    url: &str,
) -> Result<Params, KError> {
    let router = router.clone();
    let m = router.recognize(url);

    match m {
        Ok(mtch) => Ok(mtch.params().clone()),
        Err(_) => Err(KError::UrlParsing),
    }
}
