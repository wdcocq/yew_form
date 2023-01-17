use std::cell::Ref;
use std::collections::HashMap;
use std::marker::PhantomData;

use crate::Model;
use crate::{form_field::FormField, Form};
use validator::{ValidationErrors, ValidationErrorsKind};
use yew::AttrValue;

// pub enum FormState<T: Model> {
//     Owned(FormStateOwned<T>),
//     Ref(FormStateRef<T>),
// }

#[derive(PartialEq)]
pub struct FormStateOwned<T: Model> {
    pub(crate) model: T,
    fields: HashMap<AttrValue, FormField>,
}

impl<T: Model> FormStateOwned<T> {
    pub fn new(model: T) -> FormStateOwned<T> {
        let mut fields = vec![];

        model.fields("", &mut fields);

        let form = FormStateOwned {
            fields: fields
                .into_iter()
                .map(|f| (model.value(&f), f))
                .map(|(v, f)| (f.clone(), FormField::new(f, v)))
                .collect(),
            model,
        };

        form
    }

    /// This updates the model but keeps the initial field values.
    /// So differences are registered as dirty and are immediatly validated.
    pub fn update(&mut self, model: T) {
        let mut fields = vec![];
        let mut dirty = false;

        model.fields("", &mut fields);
        for field in fields {
            dirty |= self.set_value(&field, model.value(&field));
        }

        if dirty {
            self.model = model;
        }
    }

    /// Replaces with the model and resets all field states
    pub fn replace(&mut self, model: T) {
        *self = Self::new(model);
    }

    pub(crate) fn model(&self) -> &T {
        &self.model
    }

    pub(crate) fn field(&self, name: &str) -> &FormField {
        self.fields
            .get(name)
            .expect(&format!("Field {} does not exist", name))
    }

    fn field_mut(&mut self, name: &str) -> &mut FormField {
        self.fields
            .get_mut(name)
            .expect(&format!("Field {} does not exist", name))
    }

    pub(crate) fn set_value<V>(&mut self, field_name: &str, value: V) -> bool
    where
        V: Into<AttrValue> + AsRef<str>,
    {
        if self.field(field_name).value == value.as_ref() {
            return false;
        }

        let value = value.into();
        let result = self.model.set_value(field_name, &value);

        let field = self.field_mut(field_name);
        field.value = value;

        match result {
            Ok(()) => {
                self.update_validation_field(field_name);
            }
            Err(e) => {
                field.valid = false;
                field.message = e.into();
            }
        }

        true
    }

    pub fn valid(&self) -> bool {
        self.fields.values().all(FormField::valid)
    }

    pub fn dirty(&self) -> bool {
        self.fields.values().any(FormField::dirty)
    }

    pub fn field_valid(&self, field_path: &str) -> bool {
        self.field(field_path).valid
    }

    pub fn field_message(&self, field_path: &str) -> &str {
        &self.field(field_path).message
    }

    /// Marks all the fields as `dirty` and perform validation on the model
    /// Returns `true` if the model passes validation
    pub fn validate(&mut self) -> bool {
        self.update_validation();
        self.valid()
    }

    pub(crate) fn update_validation(&mut self) {
        self.clear_errors(None);

        if let Err(errors) = self.model.validate() {
            self.add_errors("", None, &errors);
        }
    }

    pub(crate) fn update_validation_field(&mut self, field: &str) {
        self.clear_errors(Some(field));

        if let Err(errors) = self.model.validate() {
            self.add_errors("", Some(field), &errors);
        }
    }

    fn clear_errors(&mut self, field: Option<&str>) {
        match field {
            Some(field) => {
                let field = self.field_mut(field);
                field.valid = true;
                field.message = Default::default();
            }
            None => {
                self.fields.values_mut().for_each(|f| {
                    f.valid = true;
                    f.message = Default::default();
                });
            }
        }
    }

    fn add_errors(
        &mut self,
        prefix: &str,
        field_name_filter: Option<&str>,
        errors: &ValidationErrors,
    ) {
        fn generate_field_name(prefix: &str, field_name: &str) -> String {
            if prefix == "" {
                String::from(field_name)
            } else {
                format!("{}.{}", prefix, field_name)
            }
        }

        for (field_name, error) in errors.errors() {
            if let Some(ref field_name_filter) = field_name_filter {
                if field_name != field_name_filter {
                    // ignore all fields not matching this field
                    continue;
                }
            }

            match error {
                ValidationErrorsKind::Struct(errors) => self.add_errors(
                    &generate_field_name(prefix, field_name),
                    field_name_filter,
                    errors,
                ),
                ValidationErrorsKind::List(_) => { /* Ignore? */ }
                ValidationErrorsKind::Field(errors) => {
                    let field = self.field_mut(&generate_field_name(prefix, field_name));

                    field.valid = false;

                    field.message = if let Some(message) = &errors[0].message {
                        match message {
                            std::borrow::Cow::Borrowed(msg) => (*msg).into(),
                            std::borrow::Cow::Owned(msg) => msg.to_owned().into(),
                        }
                    } else {
                        "Error".into()
                    }
                }
            };
        }
    }
}

// #[derive(PartialEq)]
// pub struct FormStateRef<T: Model> {
//     parent_form: Box<dyn FormLike>,
//     model_path: String,
//     fields: HashMap<AttrValue, FormField>,
//     _phantom: PhantomData<T>,
// }

// impl<P: Model, T: Model> FormStateRef<P, T> {
//     pub fn new(parent_form: Form<P>, model_path: &str) -> FormStateRef<P, T> {
//         let fields = {
//             let mut fields = vec![];

//             let model = parent_form.sub_model::<T>(model_path).unwrap();

//             model.fields("", &mut fields);

//             fields
//                 .into_iter()
//                 .map(|f| (model.value(&f), f))
//                 .map(|(v, f)| (f.clone(), FormField::new(f, v)))
//                 .collect()
//         };

//         let form = FormStateRef {
//             parent_form,
//             model_path: model_path.to_owned(),
//             fields,
//             _phantom: Default::default(),
//         };

//         form
//     }

//     /// This updates the model but keeps the initial field values.
//     /// So differences are registered as dirty and are immediatly validated.
//     // pub fn update(&mut self, model: T) {
//     //     let mut fields = vec![];
//     //     let mut dirty = false;

//     //     model.fields("", &mut fields);
//     //     for field in fields {
//     //         dirty |= self.set_value(&field, model.value(&field));
//     //     }

//     //     if dirty {
//     //         self.model = model;
//     //     }
//     // }

//     // /// Replaces with the model and resets all field states
//     // pub fn replace(&mut self, model: T) {
//     //     *self = Self::new(model);
//     // }

//     pub(crate) fn model(&self) -> Ref<T> {
//         self.parent_form.sub_model::<T>(&self.model_path).unwrap()
//     }

//     pub(crate) fn field(&self, name: &str) -> &FormField {
//         self.fields
//             .get(name)
//             .expect(&format!("Field {} does not exist", name))
//     }

//     fn field_mut(&mut self, name: &str) -> &mut FormField {
//         self.fields
//             .get_mut(name)
//             .expect(&format!("Field {} does not exist", name))
//     }

//     pub(crate) fn set_value<V>(&mut self, field_name: &str, value: V) -> bool
//     where
//         V: Into<AttrValue> + AsRef<str>,
//     {
//         if self.field(field_name).value == value.as_ref() {
//             return false;
//         }

//         // let value = value.into();
//         // let result = self.model().set_value(field_name, &value);

//         // let field = self.field_mut(field_name);
//         // field.value = value;

//         // match result {
//         //     Ok(()) => {
//         //         self.update_validation_field(field_name);
//         //     }
//         //     Err(e) => {
//         //         field.valid = false;
//         //         field.message = e.into();
//         //     }
//         // }

//         true
//     }

//     pub fn valid(&self) -> bool {
//         self.fields.values().all(FormField::valid)
//     }

//     pub fn dirty(&self) -> bool {
//         self.fields.values().any(FormField::dirty)
//     }

//     pub fn field_valid(&self, field_path: &str) -> bool {
//         self.field(field_path).valid
//     }

//     pub fn field_message(&self, field_path: &str) -> &str {
//         &self.field(field_path).message
//     }

//     /// Marks all the fields as `dirty` and perform validation on the model
//     /// Returns `true` if the model passes validation
//     pub fn validate(&mut self) -> bool {
//         self.update_validation();
//         self.valid()
//     }

//     pub(crate) fn update_validation(&mut self) {
//         self.clear_errors(None);

//         // if let Err(errors) = self.model().validate() {
//         //     self.add_errors("", None, &errors);
//         // }
//     }

//     pub(crate) fn update_validation_field(&mut self, field: &str) {
//         self.clear_errors(Some(field));

//         // if let Err(errors) = self.model().validate() {
//         //     self.add_errors("", Some(field), &errors);
//         // }
//     }

//     fn clear_errors(&mut self, field: Option<&str>) {
//         match field {
//             Some(field) => {
//                 let field = self.field_mut(field);
//                 field.valid = true;
//                 field.message = Default::default();
//             }
//             None => {
//                 self.fields.values_mut().for_each(|f| {
//                     f.valid = true;
//                     f.message = Default::default();
//                 });
//             }
//         }
//     }

//     fn add_errors(
//         &mut self,
//         prefix: &str,
//         field_name_filter: Option<&str>,
//         errors: &ValidationErrors,
//     ) {
//         fn generate_field_name(prefix: &str, field_name: &str) -> String {
//             if prefix == "" {
//                 String::from(field_name)
//             } else {
//                 format!("{}.{}", prefix, field_name)
//             }
//         }

//         for (field_name, error) in errors.errors() {
//             if let Some(ref field_name_filter) = field_name_filter {
//                 if field_name != field_name_filter {
//                     // ignore all fields not matching this field
//                     continue;
//                 }
//             }

//             match error {
//                 ValidationErrorsKind::Struct(errors) => self.add_errors(
//                     &generate_field_name(prefix, field_name),
//                     field_name_filter,
//                     errors,
//                 ),
//                 ValidationErrorsKind::List(_) => { /* Ignore? */ }
//                 ValidationErrorsKind::Field(errors) => {
//                     let field = self.field_mut(&generate_field_name(prefix, field_name));

//                     field.valid = false;

//                     field.message = if let Some(message) = &errors[0].message {
//                         match message {
//                             std::borrow::Cow::Borrowed(msg) => (*msg).into(),
//                             std::borrow::Cow::Owned(msg) => msg.to_owned().into(),
//                         }
//                     } else {
//                         "Error".into()
//                     }
//                 }
//             };
//         }
//     }
// }
