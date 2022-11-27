use crate::form_field::FormField;
use crate::form_state::FormState;
use crate::Model;
use std::borrow::BorrowMut;
use std::cell::{Cell, Ref, RefCell, RefMut};
use std::convert::AsRef;
use std::rc::Rc;
use yew::html::ImplicitClone;
use yew::prelude::*;

#[derive(Clone)]
pub struct Form<T: Model> {
    state: Rc<RefCell<FormState<T>>>,
    update_fn: Callback<()>,
    generation: Rc<Cell<u32>>,
}

impl<T: Model> ImplicitClone for Form<T> {}

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
    let update = use_force_update();

    Form {
        state: use_memo(|d| RefCell::new(FormState::new(init_fn(d))), deps),
        update_fn: Callback::from(move |_| update.force_update()),
        generation: use_memo(|_| Cell::new(0), ()),
    }
}

impl<T: Model> Form<T> {
    pub fn new<C: Into<Callback<()>>>(model: T, update_fn: C) -> Form<T> {
        Form {
            state: Rc::new(RefCell::new(FormState::new(model))),
            update_fn: update_fn.into(),
            generation: Rc::new(Cell::new(0)),
        }
    }

    fn state(&self) -> Ref<FormState<T>> {
        self.state.as_ref().borrow()
    }

    fn state_mut(&self) -> RefMut<FormState<T>> {
        self.generation.set(self.generation.get().wrapping_add(1));
        (*self.state).borrow_mut()
    }

    pub fn value<S: AsRef<str>>(&self, field: S) -> AttrValue {
        self.state().field(field.as_ref()).value().clone()
    }

    pub fn set_value<S, V>(&self, field: S, value: V)
    where
        S: AsRef<str>,
        V: Into<AttrValue> + AsRef<str>,
    {
        if self.state_mut().set_value(field.as_ref(), value) {
            self.update_fn.emit(());
        }
    }

    pub fn field<S: AsRef<str>>(&self, field: S) -> Ref<FormField> {
        Ref::map(self.state(), |s| s.field(field.as_ref()))
    }

    pub fn validate(&mut self) -> bool {
        let valid = self.state_mut().validate();
        self.update_fn.emit(());
        valid
    }

    pub fn all_valid(&self) -> bool {
        self.state().valid()
    }

    pub fn valid<S: AsRef<str>>(&self, field: S) -> bool {
        self.state().field(field.as_ref()).valid
    }

    pub fn dirty<S: AsRef<str>>(&self, field: S) -> bool {
        self.state().field(field.as_ref()).dirty
    }

    pub fn model(&self) -> Ref<T> {
        Ref::map(self.state(), |s| s.model())
    }
}

impl<T: Model> PartialEq for Form<T> {
    fn eq(&self, other: &Self) -> bool {
        self.generation == other.generation && *self.state() == *other.state()
    }
}
