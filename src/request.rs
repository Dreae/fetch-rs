use super::HttpMethod;
use super::Response;


use std::collections::hash_map::HashMap;
use std::error::Error;

use stdweb::Value;
use serde::ser::Serialize;
use serde_json;
use mediatypes;

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
    pub(crate) method: HttpMethod,
    pub(crate) url: String,
    pub(crate) body: Option<Value>,
    pub(crate) headers: HashMap<String, String>,
    pub(crate) credentials: &'static str,
    mediatype: mediatypes::MediaType
}

impl Request {
    pub fn new(url: String) -> Request {
        Request {
            method: HttpMethod::GET,
            url: url,
            body: None,
            headers: HashMap::new(),
            credentials: RequestCredentials::Omit.to_string(),
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

    pub fn body<Body: Into<Vec<u8>>>(mut self, body: Body) -> Request {
        self.body = Some(Value::String(unsafe { String::from_utf8_unchecked(body.into()) }));
        self.mediatype = mediatypes::APPLICATION_WWW_FORM_URLENCODED;

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

    pub fn send<T: FnMut(Response) + 'static>(mut self, callback: T) {
        if self.method != HttpMethod::GET && self.method != HttpMethod::HEAD {
            self.headers.insert("Content-Type".to_owned(), self.mediatype.as_ref().to_owned());
        }
        super::fetch(self, callback)
    }
}