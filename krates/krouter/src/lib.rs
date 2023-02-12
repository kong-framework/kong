use konnect::{Konnect, KustomRoute};
use kore::Kore;
use rouille::{Request, Response};

pub struct Kroute<'a> {
    kore: &'a mut Kore,
    request: &'a Request,
    static_files_path: &'a str,
    kustom_routes: Vec<KustomRoute<'a>>,
}

impl<'a> Kroute<'a> {
    pub fn new(
        kore: &'a mut Kore,
        request: &'a Request,
        static_files_path: &'a str,
        kustom_routes: Vec<KustomRoute<'a>>,
    ) -> Self {
        Kroute {
            kore,
            request,
            static_files_path,
            kustom_routes,
        }
    }

    pub fn kroute(&mut self) -> Response {
        // Route built in APIs
        match self.request.url().as_str() {
            konnect::accounts::ADDRESS => {
                return konnect::accounts::Accounts::handle_request(self.kore, self.request);
            }
            _ => (),
        }

        // Route user provided APIs
        for route in &self.kustom_routes {
            if route.address == self.request.url().as_str() {
                return (route.handler)(self.kore, self.request);
            }
        }

        let response = rouille::match_assets(self.request, self.static_files_path);

        if response.is_success() {
            return response;
        }

        Response::html("404 error").with_status_code(404)
    }
}
