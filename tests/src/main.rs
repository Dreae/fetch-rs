#[macro_use]
extern crate stdweb;
extern crate fetch;

#[macro_use]
extern crate serde_derive;
extern crate serde;

use stdweb::{Value, Reference};
use stdweb::web::TypedArray;

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Post {
    userId: i32,
    id: i32,
    title: String,
    body: String
}

js_serializable!(Post);

fn test_fetch() {
    fetch::get("http://jsonplaceholder.typicode.com/posts/1", |res| {
        stdweb::web::alert(&format!("{}", res.status().is_ok()));
        let post: Post = res.json().unwrap();
        js! {
            console.log(@{post});
        };
    });

    let post = Post {
        userId: 1,
        id: 2,
        title: "Testing".to_owned(),
        body: "Testme".to_owned()
    };

    let req = fetch::post("http://jsonplaceholder.typicode.com/posts").json(&post);
    req.send(|res| {
        js! {
            console.log(@{res.text()});
        }
    });

    fetch::post("http://jsonplaceholder.typicode.com/posts").body("Hello!").send(|res| {
        js! {
            console.log(@{res.text()});
        }
    })
}

fn run() {
    js! {
        window.test_fetch = @{test_fetch};
    }

    test_fetch();
}

#[derive(Serialize)]
struct ReferenceHolder {
    reference: Value
}
js_serializable!(ReferenceHolder);

fn test_ref() {
    let message = "Hello!".as_bytes();
    let reference = Value::Reference(Reference::from(TypedArray::from(message)));
    let obj = ReferenceHolder {
        reference: Value::Reference(Reference::from(TypedArray::from(message)))
    };

    js! {
        console.log(@{reference});
        console.log(@{obj});
    };
}

fn main() {
    stdweb::initialize();

    test_ref();

    js! {
        Module.exports.run = @{run};
    }
}
