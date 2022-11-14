use crate::form_field::FormField;
use crate::form_state::FormState;
use crate::Model;
use std::borrow::Borrow;
use std::cell::{Ref, RefCell, RefMut};
use std::convert::AsRef;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use yew::html::ImplicitClone;
use yew::platform::spawn_local;
use yew::prelude::*;

#[derive(Clone)]
pub struct Form<T: Model> {
    state: Rc<RefCell<FormState<T>>>,
    update_fn: Callback<()>,
}

impl<T: Model> ImplicitClone for Form<T> {}

pub struct FormFieldMutRef<'a> {
    field: Option<RefMut<'a, FormField>>,
    update: Callback<RefMut<'a, FormField>>,
}

impl<'a> Deref for FormFieldMutRef<'a> {
    type Target = FormField;

    fn deref(&self) -> &Self::Target {
        self.field.as_ref().unwrap().deref()
    }
}

impl<'a> DerefMut for FormFieldMutRef<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.field.as_mut().unwrap().deref_mut()
    }
}

impl Drop for FormFieldMutRef<'_> {
    fn drop(&mut self) {
        let field = self.field.take().unwrap();
        self.update.emit(field);
    }
}

#[hook]
pub fn use_form<T: Model>(init_fn: impl FnOnce() -> T) -> Form<T> {
    let update = use_force_update();
    Form {
        state: use_mut_ref(move || FormState::new(init_fn())),
        update_fn: Callback::from(move |_| update.force_update()),
    }
}

impl<T: Model> Form<T> {
    pub fn new(model: T, update_fn: Callback<()>) -> Form<T> {
        Form {
            state: Rc::new(RefCell::new(FormState::new(model))),
            update_fn,
        }
    }

    fn state(&self) -> Ref<FormState<T>> {
        self.state.as_ref().borrow()
    }

    fn state_mut(&self) -> RefMut<FormState<T>> {
        self.state.borrow_mut()
    }

    pub fn field<S: AsRef<str>>(&self, field_name: S) -> Ref<FormField> {
        Ref::map(self.state(), |s| s.field(field_name.as_ref()))
    }

    pub fn field_mut<S: AsRef<str>>(&self, field_name: S) -> FormFieldMutRef {
        let field = RefMut::map(self.state_mut(), |s| s.field_mut(field_name.as_ref()));
        let initial = field.value().clone();
        let form = self.clone();
        let update_fn = self.update_fn.clone();

        FormFieldMutRef {
            field: Some(field),
            update: Callback::from(move |field: RefMut<FormField>| {
                let value = field.value();
                if *value != initial {
                    let value = value.clone();
                    let field_name = field.name().clone();
                    drop(field);
                    form.state_mut()
                        .model_mut()
                        .set_value(&field_name, &value)
                        .expect("Couldn't convert");
                    form.state_mut().update_validation_field(&field_name);
                    update_fn.emit(());
                }
            }),
        }
    }

    pub fn validate(&mut self) -> bool {
        let valid = self.state_mut().validate();
        self.update_fn.emit(());
        valid
    }

    pub fn valid(&self) -> bool {
        self.state().valid()
    }

    pub fn model(&self) -> Ref<T> {
        Ref::map(self.state(), |s| s.model())
    }
}

impl<T: Model> PartialEq for Form<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.state, &other.state) || self.state().model == other.state().model
    }

    fn ne(&self, other: &Self) -> bool {
        self.state().model != other.state().model
    }
}
