use yew::AttrValue;

#[derive(PartialEq)]
pub struct FormField {
    pub(crate) name: AttrValue,
    pub(crate) value: AttrValue,
    pub(crate) message: AttrValue,
    pub(crate) dirty: bool,
    pub(crate) valid: bool,
}

impl FormField {
    pub fn new(name: impl Into<AttrValue>, value: impl Into<AttrValue>) -> Self {
        FormField {
            name: name.into(),
            value: value.into(),
            message: Default::default(),
            dirty: false,
            valid: true,
        }
    }

    pub fn name(&self) -> &AttrValue {
        &self.name
    }

    pub fn value(&self) -> &AttrValue {
        &self.value
    }

    pub fn message(&self) -> &AttrValue {
        &self.message
    }

    pub fn dirty(&self) -> bool {
        self.dirty
    }

    pub fn valid(&self) -> bool {
        self.valid
    }
}
