use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
    str::FromStr,
};
use validator::Validate;
use yew::AttrValue;

pub trait FormValue {
    fn fields(&self, prefix: &str, fields: &mut Vec<AttrValue>) {
        // By default, announce the value to be a scalar
        fields.push(prefix.to_owned().into());
    }
    fn value(&self, field_path: &str) -> AttrValue;
    fn set_value(&mut self, field_path: &str, value: &str) -> Result<(), &'static str>;
    // fn field<T>(&self, field_path: &str) -> Option<&T>
    // where
    //     T: FormValue + 'static,
    //     Self: Sized + 'static,
    // {
    //     use std::any::Any;
    //     let this: &dyn Any = self;
    //     this.downcast_ref::<T>()
    // }
}

pub trait Model: FormValue + Validate + PartialEq + 'static {}

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
                    debug_assert!(field_path.is_empty());
                    self.to_string().into()
                }

                fn set_value(&mut self, field_path: &str, value: &str) -> Result<(), &'static str> {
                    debug_assert!(field_path.is_empty());

                    match value.parse::<$t>() {
                        Ok(v) => {
                            *self = v;
                            Ok(())
                        }
                        Err(_) => Err("Could not convert value to type")
                    }
                }

            }
        )+
    };
}

impl_form_value!(
    bool, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, char, String
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

        match value.parse::<T>() {
            Ok(v) => {
                self.0 = v;
                Ok(())
            }
            Err(_) => Err("Could not convert value to type"),
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
    T: FormValue + Default,
{
    fn value(&self, field_path: &str) -> AttrValue {
        match self {
            Some(value) => value.value(field_path),
            None => Default::default(),
        }
    }

    fn set_value(&mut self, field_path: &str, value: &str) -> Result<(), &'static str> {
        if value.is_empty() {
            *self = None;
            Ok(())
        } else {
            let entry = self.get_or_insert_with(Default::default);
            entry.set_value(field_path, value)
        }
    }
}

impl<T> FormValue for [T]
where
    T: FormValue,
{
    fn fields(&self, prefix: &str, fields: &mut Vec<AttrValue>) {
        let field_prefix = if prefix.is_empty() {
            String::default()
        } else {
            format!("{prefix}.")
        };

        for (i, m) in self.iter().enumerate() {
            m.fields(&format!("{field_prefix}{i}"), fields);
        }
    }

    fn value(&self, field_path: &str) -> AttrValue {
        let (field_name, suffix) = split_field_path(field_path);
        let index: usize = field_name
            .parse()
            .expect(&format!("Can't convert `{field_name}` to a valid index`"));

        let entry = self
            .get(index)
            .expect(&format!("Index `{index}` is out of bounds"));

        entry.value(suffix)
    }

    fn set_value(&mut self, field_path: &str, value: &str) -> Result<(), &'static str> {
        let (field_name, suffix) = split_field_path(field_path);

        let index: usize = field_name
            .parse()
            .expect(&format!("Can't convert `{field_name}` to a valid index`"));

        let entry = self
            .get_mut(index)
            .expect(&format!("Index `{index}` is out of bounds"));

        entry.set_value(suffix, value)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     use crate as yew_form;
//     use yew_form_derive::Model;

//     #[test]
//     fn test_split_field_path() {
//         let (field, suffix) = split_field_path("field");
//         assert_eq!(field, "field");
//         assert_eq!(suffix, "");

//         let (field, suffix) = split_field_path("field.sub");
//         assert_eq!(field, "field");
//         assert_eq!(suffix, "sub");

//         let (field, suffix) = split_field_path("field.sub.subsub");
//         assert_eq!(field, "field");
//         assert_eq!(suffix, "sub.subsub");
//     }

//     #[test]
//     fn test_form_value() -> Result<(), &'static str> {
//         fn test_value<T>(test_str_value: &str, test_value: T) -> Result<(), &'static str>
//         where
//             T: FormValue + Default + PartialEq + Debug,
//         {
//             let mut v = Default::default();
//             T::set_value(&mut v, "", test_str_value)?;
//             assert_eq!(v, test_value);
//             assert_eq!(test_value.value("").as_str(), test_str_value);
//             Ok(())
//         }

//         test_value("true", true)?;
//         test_value("false", false)?;
//         test_value("42", 42u8)?;
//         test_value("42", 42u16)?;
//         test_value("42", 42u32)?;
//         test_value("42", 42u64)?;
//         test_value("42", 42u128)?;
//         test_value("42", 42usize)?;
//         test_value("42", 42i8)?;
//         test_value("42", 42i16)?;
//         test_value("42", 42i32)?;
//         test_value("42", 42i64)?;
//         test_value("42", 42i128)?;
//         test_value("42", 42isize)?;
//         test_value("4.2", 4.2f32)?;
//         test_value("4.2", 4.2f64)?;
//         test_value("a", 'a')?;
//         test_value("42", String::from("42"))?;
//         test_value::<Option<u8>>("", None)?;
//         test_value::<Option<u8>>("42", Some(42))
//     }

//     #[test]
//     fn test_form_value_vec() -> Result<(), &'static str> {
//         #[derive(Model, Validate, PartialEq)]
//         struct Parent {
//             ids: Vec<u32>,
//             opt_ids: Vec<Option<u32>>,
//             children: Vec<Child>,
//             opt_children: Vec<Option<Child>>,
//         }

//         #[derive(Model, Validate, PartialEq, Default)]
//         struct Child {
//             name: String,
//         }

//         let mut parent = Parent {
//             ids: vec![0, 1],
//             opt_ids: vec![Some(0), None],
//             children: vec![Child { name: "a".into() }, Child { name: "b".into() }],
//             opt_children: vec![
//                 Some(Child {
//                     name: "maybe".into(),
//                 }),
//                 None,
//             ],
//         };

//         assert_eq!(parent.value("ids.0").as_str(), "0");
//         assert_eq!(parent.value("ids.1").as_str(), "1");
//         parent.set_value("ids.0", "2")?;
//         parent.set_value("ids.1", "3")?;
//         assert_eq!(parent.value("ids.0").as_str(), "2");
//         assert_eq!(parent.value("ids.1").as_str(), "3");

//         assert_eq!(parent.value("opt_ids.0").as_str(), "0");
//         assert_eq!(parent.value("opt_ids.1").as_str(), "");
//         parent.set_value("opt_ids.0", "")?;
//         parent.set_value("opt_ids.1", "1")?;
//         assert_eq!(parent.value("opt_ids.0").as_str(), "");
//         assert_eq!(parent.value("opt_ids.1").as_str(), "1");

//         assert_eq!(parent.value("children.0.name").as_str(), "a");
//         assert_eq!(parent.value("children.1.name").as_str(), "b");
//         parent.set_value("children.0.name", "first")?;
//         parent.set_value("children.1.name", "second")?;
//         assert_eq!(parent.value("children.0.name").as_str(), "first");
//         assert_eq!(parent.value("children.1.name").as_str(), "second");

//         assert_eq!(parent.value("opt_children.0.name").as_str(), "maybe");
//         assert_eq!(parent.value("opt_children.1.name").as_str(), "");
//         parent.set_value("opt_children.0.name", "")?;
//         parent.set_value("opt_children.1.name", "not")?;
//         assert_eq!(parent.value("opt_children.0.name").as_str(), "");
//         assert_eq!(parent.value("opt_children.1.name").as_str(), "not");

//         Ok(())
//     }

//     #[test]
//     #[should_panic]
//     fn test_form_value_vec_fail() {
//         #[derive(Model, Validate, PartialEq)]
//         struct Parent {
//             ids: Vec<u32>,
//         }

//         let parent = Parent { ids: vec![0, 1] };

//         parent.value("ids.2");
//     }

//     #[test]
//     fn test_form_value_array() -> Result<(), &'static str> {
//         #[derive(Model, Validate, PartialEq)]
//         struct Test {
//             ids: [u32; 2],
//         }

//         let mut test = Test { ids: [0, 1] };

//         assert_eq!(test.value("ids.0").as_str(), "0");
//         assert_eq!(test.value("ids.1").as_str(), "1");
//         test.set_value("ids.0", "2")?;
//         test.set_value("ids.1", "3")?;
//         assert_eq!(test.value("ids.0").as_str(), "2");
//         assert_eq!(test.value("ids.1").as_str(), "3");

//         Ok(())
//     }
// }
