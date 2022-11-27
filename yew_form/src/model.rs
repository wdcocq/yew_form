use std::str::FromStr;
use validator::Validate;
use yew::AttrValue;

pub trait FormValue {
    fn fields(&self, prefix: &str, fields: &mut Vec<AttrValue>) {
        // By default, announce the value to be a scalar
        fields.push(prefix.to_owned().into());
    }
    fn value(&self, field_path: &str) -> AttrValue;
    fn set_value(&mut self, field_path: &str, value: &str) -> Result<(), &'static str>;
}

pub trait Model: FormValue + Validate + PartialEq + Clone + 'static {}

pub fn split_field_path(field_path: &str) -> (&str, &str) {
    if let Some(index) = field_path.find(".") {
        (&field_path[0..index], &field_path[index + 1..])
    } else {
        (field_path, "")
    }
}

impl<T: ToString + FromStr> FormValue for T {
    fn value(&self, field_path: &str) -> AttrValue {
        debug_assert!(field_path == "");

        self.to_string().into()
    }

    fn set_value(&mut self, field_path: &str, value: &str) -> Result<(), &'static str> {
        debug_assert!(field_path == "");

        if let Ok(v) = value.parse::<T>() {
            *self = v;
            Ok(())
        } else {
            Err("Could not convert")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::split_field_path;

    #[test]
    fn test_split_field_path() {
        let path = "field";
        let (field, suffix) = split_field_path(path);

        assert_eq!(field, "field");
        assert_eq!(suffix, "");

        let path = "field.sub";
        let (field, suffix) = split_field_path(path);

        assert_eq!(field, "field");
        assert_eq!(suffix, "sub");

        let path = "field.sub.subsub";
        let (field, suffix) = split_field_path(path);

        assert_eq!(field, "field");
        assert_eq!(suffix, "sub.subsub");
    }
}
