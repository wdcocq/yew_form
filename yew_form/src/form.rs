use crate::form_field::FormField;
use crate::form_state::FormState;
use crate::Model;
use std::cell::{Cell, Ref, RefCell, RefMut};
use std::convert::AsRef;
use std::fmt::Debug;
use std::rc::Rc;
use yew::html::ImplicitClone;
use yew::prelude::*;

#[derive(Clone)]
pub struct Form<T: Model> {
    state: Rc<RefCell<FormState<T>>>,
    generation: UseStateHandle<u32>,
}

impl<T: Model> ImplicitClone for Form<T> {}

impl<T> Debug for Form<T>
where
    T: Model + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Form")
            .field("model", &self.model())
            .finish()
    }
}
#[hook]
pub fn use_form<T>(init_fn: impl FnOnce() -> T) -> Form<T>
where
    T: Model,
{
    use_form_with_deps(|_| init_fn(), ())
}

#[hook]
pub fn use_form_with_deps<T, D>(init_fn: impl FnOnce(&D) -> T, deps: D) -> Form<T>
where
    T: Model,
    D: PartialEq + 'static,
{
    Form {
        state: use_memo(|d| RefCell::new(FormState::new(init_fn(d))), deps),
        generation: use_state(|| 0),
    }
}

impl<T: Model> Form<T> {
    fn state(&self) -> Ref<FormState<T>> {
        self.state.as_ref().borrow()
    }

    fn state_mut(&self) -> RefMut<FormState<T>> {
        (*self.state).borrow_mut()
    }

    pub fn value<S: AsRef<str>>(&self, field: S) -> Ref<AttrValue> {
        Ref::map(self.field(field.as_ref()), |f| f.value())
    }

    /// Returns [`Some(..)`] when the field is dirty, else returns [`None`]
    pub fn dirty_value<S: AsRef<str>>(&self, field: S) -> Option<Ref<AttrValue>> {
        let field = self.field(field.as_ref());
        field.dirty().then(|| Ref::map(field, |f| f.value()))
    }

    pub fn set_value<S, V>(&self, field: S, value: V)
    where
        S: AsRef<str>,
        V: Into<AttrValue> + AsRef<str>,
    {
        if self.state_mut().set_value(field.as_ref(), value) {
            self.inc_generation()
        }
    }

    pub fn field<S: AsRef<str>>(&self, field: S) -> Ref<FormField> {
        Ref::map(self.state(), |s| s.field(field.as_ref()))
    }

    pub fn validate(&self) -> bool {
        let valid = self.state_mut().validate();
        self.inc_generation();
        valid
    }

    pub fn valid(&self) -> bool {
        self.state().valid()
    }

    pub fn dirty(&self) -> bool {
        self.state().dirty()
    }

    pub fn model(&self) -> Ref<T> {
        Ref::map(self.state(), |s| s.model())
    }

    /// This updates the model but keeps the initial field values.
    /// So differences are registered as dirty and are immediatly validated.
    pub fn update(&self, model: &T) {
        self.state_mut().update(model);
        self.inc_generation();
    }

    fn inc_generation(&self) {
        self.generation.set((*self.generation).wrapping_add(1));
    }
}

impl<T: Model> PartialEq for Form<T> {
    fn eq(&self, other: &Self) -> bool {
        *self.generation == *other.generation && Rc::ptr_eq(&self.state, &other.state)
    }
}
