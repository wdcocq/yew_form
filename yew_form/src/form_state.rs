use std::collections::HashMap;

use crate::form_field::FormField;
use crate::Model;
use validator::{ValidationErrors, ValidationErrorsKind};
use yew::AttrValue;

#[derive(PartialEq)]
pub struct FormState<T: Model> {
    pub(crate) model: T,
    fields: HashMap<AttrValue, FormField>,
}

impl<T: Model> FormState<T> {
    pub fn new(model: T) -> FormState<T> {
        let mut fields = vec![];

        model.fields("", &mut fields);

        let form = FormState {
            model: model.clone(),
            fields: fields
                .into_iter()
                .map(|f| (model.value(&f), f))
                .map(|(v, f)| (f.clone(), FormField::new(f, v)))
                .collect(),
        };

        form
    }

    pub fn model(&self) -> &T {
        &self.model
    }

    pub fn model_mut(&mut self) -> &mut T {
        &mut self.model
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

    pub fn set_value<V>(&mut self, field_name: &str, value: V) -> bool
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
        field.dirty = true;

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
        self.fields.values().all(|f| f.valid)
    }

    pub fn dirty(&self) -> bool {
        self.fields.values().any(|f| f.dirty)
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
        self.fields.values_mut().for_each(|f| {
            f.valid = true;
            f.dirty = true;
        });

        self.update_validation();

        self.valid()
    }

    pub(crate) fn update_validation(&mut self) {
        match self.model.validate() {
            Ok(()) => self.clear_errors(None),
            Err(errors) => {
                self.add_errors("", None, &errors);
            }
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
