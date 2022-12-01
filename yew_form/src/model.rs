use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
    str::FromStr,
};
use strum::Display;
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

macro_rules! impl_form_value {
    ($($t:ty),+) => {
        $(
            impl FormValue for $t {
                fn value(&self, field_path: &str) -> AttrValue {
                    debug_assert!(field_path == "");
                    self.to_string().into()
                }

                fn set_value(&mut self, field_path: &str, value: &str) -> Result<(), &'static str> {
                    debug_assert!(field_path == "");

                    if let Ok(v) = value.parse::<$t>() {
                        *self = v;
                        Ok(())
                    } else {
                        Err("Could not convert")
                    }
                }
            }
        )+
    };
}

impl_form_value!(
    bool, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, String
);

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FormValueWrapper<T>(pub T);

impl<T> Display for FormValueWrapper<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> FormValue for FormValueWrapper<T>
where
    T: ToString + FromStr,
{
    fn value(&self, field_path: &str) -> AttrValue {
        debug_assert!(field_path == "");
        self.0.to_string().into()
    }

    fn set_value(&mut self, field_path: &str, value: &str) -> Result<(), &'static str> {
        debug_assert!(field_path == "");

        if let Ok(v) = value.parse::<T>() {
            self.0 = v;
            Ok(())
        } else {
            Err("Could not convert")
        }
    }
}

impl FormValue for AttrValue {
    fn value(&self, field_path: &str) -> AttrValue {
        debug_assert!(field_path == "");
        self.clone()
    }

    fn set_value(&mut self, field_path: &str, value: &str) -> Result<(), &'static str> {
        debug_assert!(field_path == "");
        *self = value.to_string().into();
        Ok(())
    }
}

impl<T> Deref for FormValueWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for FormValueWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for FormValueWrapper<T> {
    fn from(t: T) -> Self {
        FormValueWrapper(t)
    }
}

impl<T> FormValue for Option<T>
where
    T: FormValue + ToString + FromStr,
{
    fn value(&self, field_path: &str) -> AttrValue {
        match self {
            Some(value) => value.value(field_path),
            None => Default::default(),
        }
    }

    fn set_value(&mut self, field_path: &str, value: &str) -> Result<(), &'static str> {
        debug_assert!(field_path == "");

        if value.is_empty() {
            *self = None;
            Ok(())
        } else if let Ok(v) = value.parse::<T>() {
            *self = Some(v);
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
