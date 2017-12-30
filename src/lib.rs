#![feature(try_from)]
#![recursion_limit="512"]

#[macro_use]
extern crate stdweb;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate mime;

mod response;
mod status;
mod request;

pub mod mediatypes;

pub use response::Response;
pub use status::StatusCode;
pub use request::{Request, RequestCredentials};

#[derive(PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD
}

impl HttpMethod {
    pub fn as_string(&self) -> &'static str {
        match *self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::HEAD => "HEAD"
        }
    }
}

#[inline]
pub fn fetch<F: FnMut(Response) + 'static>(request: Request, callback: F) {
    js! {
        let opts = {
            method: @{request.method.as_string()}
        };

        let body = @{request.body};
        if (body !== null) {
            opts.body = body
        }

        let headers = @{request.headers};
        if (headers !== null) {
            opts.headers = headers;
        }

        fetch(@{request.url}, opts).then((res) => {
            let cb = @{callback};
            let rust_resp = {};
            rust_resp.ok = res.ok;
            rust_resp.status_code = res.status;
            rust_resp.status_text = res.statusText;

            res.text().then((txt) => {
                rust_resp.text = txt;
                if (cb !== null) {
                    cb(rust_resp);

                    cb.drop();
                }
            });
        });
    };
}

pub fn get<F: FnMut(Response) + 'static>(url: &str, callback: F) {
    let request = Request::new(url.to_owned());

    fetch(request, callback);
}

pub fn post(url: &str) -> Request {
    let mut request = Request::new(url.to_owned());
    request.method = HttpMethod::POST;

    return request;
}