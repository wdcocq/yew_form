use yew::AttrValue;

pub struct FormField {
    pub field_name: AttrValue,
    pub field_value: AttrValue,
    pub message: AttrValue,
    pub dirty: bool,
    pub valid: bool,
}

impl FormField {
    pub fn new<T>(field_name: T, field_value: T) -> Self
    where
        T: Into<AttrValue>,
    {
        FormField {
            field_name: field_name.into(),
            field_value: field_value.into(),
            message: Default::default(),
            dirty: false,
            valid: true,
        }
    }

    pub fn value(&self) -> &AttrValue {
        &self.field_value
    }

    pub fn set_value(&mut self, value: impl Into<AttrValue>) {
        self.field_value = value.into();
    }

    pub fn name(&self) -> &AttrValue {
        &self.field_name
    }
}
