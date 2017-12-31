use super::HttpMethod;
use super::Response;


use std::collections::hash_map::HashMap;
use std::error::Error;

use stdweb::{Value, Reference};
use stdweb::web::TypedArray;
use serde::ser::Serialize;
use serde_json;
use mediatypes;
use super::FormData;
use super::QueryString;

pub enum RequestCredentials {
    Omit,
    SameOrigin,
    Include
}

impl RequestCredentials {
    fn to_string(&self) -> &'static str {
        match *self {
            RequestCredentials::Omit => "omit",
            RequestCredentials::SameOrigin => "same-origin",
            RequestCredentials::Include => "include"
        }
    }
}

pub struct Request {
    method: HttpMethod,
    url: String,
    body: Option<Value>,
    headers: HashMap<String, String>,
    credentials: RequestCredentials,
    mediatype: mediatypes::MediaType
}

#[derive(Serialize)]
pub struct RequestOptions {
    method: &'static str,
    headers: HashMap<String, String>,
    credentials: &'static str
}
js_serializable!(RequestOptions);

impl Request {
    pub fn new(url: String, method: HttpMethod) -> Request {
        Request {
            method,
            url,
            body: None,
            headers: HashMap::new(),
            credentials: RequestCredentials::Omit,
            mediatype: mediatypes::TEXT_PLAIN_UTF_8
        }
    }

    pub fn url(&self) -> &str {
        return self.url.as_ref();
    }

    pub fn content_type(mut self, media_type: mediatypes::MediaType) -> Request {
        self.mediatype = media_type;

        self
    }

    pub fn set_header(mut self, header: String, value: String) -> Request {
        self.headers.insert(header, value);

        self
    }

    pub fn get_header(&self, header: String) -> Option<&String> {
        self.headers.get(&header)
    }

    pub fn body<Body: Into<Vec<u8>>>(mut self, body: Body) -> Request {
        let typed_array = TypedArray::from(&body.into()[..]);
        let body_ref = Value::Reference(Reference::from(typed_array));

        self.body = Some(body_ref);
        self.mediatype = mediatypes::APPLICATION_OCTET_STREAM;

        self
    }

    pub fn json<Body: Serialize>(mut self, body: &Body) -> Request {
        let serialized = serde_json::to_string(body);
        match serialized {
            Ok(buf) => {
                self.body = Some(Value::String(buf));
                self.mediatype = mediatypes::APPLICATION_JSON;
            },
            Err(msg) => {
                js! {
                    console.err("Error serializing body", @{msg.description()});
                }
            }
        }

        self
    }

    pub fn text(mut self, body: String) -> Request {
        self.body = Some(Value::String(body));

        self
    }

    pub fn form(mut self, body: QueryString) -> Request {
        self.body = Some(body.into());
        self.mediatype = mediatypes::APPLICATION_WWW_FORM_URLENCODED;

        self
    }

    pub fn multipart_form(mut self, body: FormData) -> Request {
        self.body = Some(body.into());
        self.mediatype = mediatypes::MULTIPART_FORM_DATA;

        self
    }

    pub fn send<T: FnMut(Response) + 'static>(self, callback: T) {
        super::fetch(self, callback)
    }

    // Return body seperately, there seems to be a bug serializing References in
    // stdweb if they're contained in an object (#61).
    pub(crate) fn build_options(mut self) -> (RequestOptions, Option<Value>) {
        if self.method != HttpMethod::GET && self.method != HttpMethod::HEAD {
            self.headers.insert("Content-Type".to_owned(), self.mediatype.as_ref().to_owned());
        }

        let opts = RequestOptions {
            method: self.method.as_string(),
            headers: self.headers,
            credentials: self.credentials.to_string()
        };

        (opts, self.body)
    }
}