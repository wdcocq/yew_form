use crate::form_field::FormField;
use crate::form_state::FormStateOwned;
use crate::Model;
use std::cell::{Cell, Ref, RefCell, RefMut};
use std::convert::AsRef;
use std::fmt::Debug;
use std::rc::Rc;
use yew::html::ImplicitClone;
use yew::prelude::*;

pub trait FormLike<T: Model>: Clone + PartialEq {
    fn value<S: AsRef<str>>(&self, field: S) -> Ref<AttrValue>;

    /// Returns [`Some(..)`] when the field is dirty, else returns [`None`]
    fn dirty_value<S: AsRef<str>>(&self, field: S) -> Option<Ref<AttrValue>>;

    fn set_value<S, V>(&self, field: S, value: V)
    where
        S: AsRef<str>,
        V: Into<AttrValue> + AsRef<str>;

    fn field<S: AsRef<str>>(&self, field: S) -> Ref<FormField>;
    fn validate(&self) -> bool;
    fn valid(&self) -> bool;
    fn dirty(&self) -> bool;
    fn model(&self) -> Ref<T>;
    // fn child_model<T: Model>(&self, field_path: &str) -> Ref<T>;
}

pub struct Form<T: Model> {
    state: Rc<RefCell<FormStateOwned<T>>>,
    generation: UseStateHandle<u32>,
}

impl<T: Model> Clone for Form<T> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            generation: self.generation.clone(),
        }
    }
}

impl<T: Model> ImplicitClone for Form<T> {}

impl<T> Debug for Form<T>
where
    T: Model + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Form")
            .field("model", &*self.model())
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
        state: use_memo(|d| RefCell::new(FormStateOwned::new(init_fn(d))), deps),
        generation: use_state(|| 0),
    }
}

impl<T: Model> Form<T> {
    fn state(&self) -> Ref<FormStateOwned<T>> {
        self.state.as_ref().borrow()
    }

    fn state_mut(&self) -> RefMut<FormStateOwned<T>> {
        (*self.state).borrow_mut()
    }

    fn inc_generation(&self) {
        self.generation.set((*self.generation).wrapping_add(1));
    }

    // pub(crate) fn sub_model<M>(&self, field_path: &str) -> Option<Ref<M>>
    // where
    //     M: Model,
    // {
    //     Ref::filter_map(self.model(), |m| m.field(field_path)).ok()
    // }
}

impl<T: Model> FormLike<T> for Form<T> {
    fn value<S: AsRef<str>>(&self, field: S) -> Ref<AttrValue> {
        Ref::map(self.field(field.as_ref()), |f| f.value())
    }

    /// Returns [`Some(..)`] when the field is dirty, else returns [`None`]
    fn dirty_value<S: AsRef<str>>(&self, field: S) -> Option<Ref<AttrValue>> {
        let field = self.field(field.as_ref());
        field.dirty().then(|| Ref::map(field, |f| f.value()))
    }

    fn set_value<S, V>(&self, field: S, value: V)
    where
        S: AsRef<str>,
        V: Into<AttrValue> + AsRef<str>,
    {
        if self.state_mut().set_value(field.as_ref(), value) {
            self.inc_generation()
        }
    }

    fn field<S: AsRef<str>>(&self, field: S) -> Ref<FormField> {
        Ref::map(self.state(), |s| s.field(field.as_ref()))
    }

    fn validate(&self) -> bool {
        let valid = self.state_mut().validate();
        self.inc_generation();
        valid
    }

    fn valid(&self) -> bool {
        self.state().valid()
    }

    fn dirty(&self) -> bool {
        self.state().dirty()
    }

    fn model(&self) -> Ref<T> {
        Ref::map(self.state(), |s| s.model())
    }

    // fn model(&self) -> Ref<T> {
    //     Ref::map(self.state(), |s| s.model())
    // }
    // fn update(&self, model: T) {
    //     self.state_mut().update(model);
    //     self.inc_generation();
    // }

    // fn replace(&self, model: T) {
    //     self.state_mut().replace(model);
    //     self.inc_generation();
    // }
}

impl<T: Model> PartialEq for Form<T> {
    fn eq(&self, other: &Self) -> bool {
        *self.generation == *other.generation && Rc::ptr_eq(&self.state, &other.state)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate as yew_form;
//     use validator::Validate;
//     use yew_form_derive::Model;

//     #[test]
//     fn test_form_ref() {
//         #[derive(Model, Validate, PartialEq)]
//         struct Parent {
//             child: Child,
//         }

//         #[derive(Model, Validate, PartialEq)]
//         struct Child {
//             name: String,
//         }
//     }
// }
