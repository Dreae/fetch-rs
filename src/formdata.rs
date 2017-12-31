use stdweb::Value;

pub struct FormData {
    data: Value
}

impl FormData {
    pub fn new() -> FormData {
        let form_data = js! {
            return new FormData();
        };

        FormData {
            data: form_data
        }
    }

    pub fn append<T: Into<Value>>(&self, key: &str, value: T) {
        js! {
            let form_data = @{&self.data};
            form_data.append(@{key}, @{&value.into()});
        }
    }
}

impl Into<Value> for FormData {
    fn into(self) -> Value {
        return self.data;
    }
}