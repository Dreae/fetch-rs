use stdweb::Value;

pub struct QueryString {
    search_params: Value
}

impl QueryString {
    pub fn new() -> QueryString {
        let search_params = js! {
            return new URLSearchParams();
        };

        QueryString {
            search_params
        }
    }

    pub fn append<T: Into<Value>>(&self, key: &str, value: T) {
        js! {
            let search_params = @{&self.search_params};
            search_params.append(@{key}, @{value.into()});
        }
    }

    pub fn set<T: Into<Value>>(&self, key: &str, value: T) {
        js! {
            let search_params = @{&self.search_params};
            search_params.set(@{key}, @{value.into()});
        }
    }
}

impl Into<Value> for QueryString {
    fn into(self) -> Value {
        self.search_params
    }
}

impl Into<String> for QueryString {
    fn into(self) -> String {
        let string = js! {
            let params = @{self.search_params};
            return params.toString();
        };

        string.into_string().unwrap()
    }
}