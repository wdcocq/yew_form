use yew::AttrValue;

#[derive(PartialEq)]
pub struct FormField {
    pub(crate) name: AttrValue,
    pub(crate) value: AttrValue,
    pub(crate) initial: AttrValue,
    pub(crate) message: AttrValue,
    pub(crate) valid: bool,
}

impl FormField {
    pub fn new(name: impl Into<AttrValue>, value: impl Into<AttrValue>) -> Self {
        let value = value.into();

        FormField {
            name: name.into(),
            value: value.clone(),
            initial: value.clone(),
            message: Default::default(),
            valid: true,
        }
    }

    pub fn name(&self) -> &AttrValue {
        &self.name
    }

    pub fn value(&self) -> &AttrValue {
        &self.value
    }

    pub fn initial_value(&self) -> &AttrValue {
        &self.initial
    }

    pub fn message(&self) -> &AttrValue {
        &self.message
    }

    pub fn dirty(&self) -> bool {
        self.initial != self.value
    }

    pub fn valid(&self) -> bool {
        self.valid
    }
}
