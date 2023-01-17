use std::{
    borrow::Borrow,
    cell::{Ref, RefCell},
    collections::HashMap,
    marker::PhantomData,
    ops::Deref,
    rc::Rc,
};

use yew::AttrValue;

pub trait ModelPath {}

pub trait PrefixPath<T>: Sized {
    fn prefix(self, path: T) -> Self;
}

// pub trait SuffixPath<P> {
//     fn prefix(suffix: Self) -> P;
// }

// impl<P> SuffixPath<P> for P
// where
//     P: ModelPath,
// {
//     fn prefix(suffix: Self) -> P {
//         suffix
//     }
// }

// impl<P, S> SuffixPath<S> for P
// where
//     S: SuffixPath<T = SuffixPath<P>>,
// {
//     fn prefix(suffix: Self) -> S {
//         S::prefix(T::prefix(suffix))
//     }
// }

// impl<P> PrefixPath<P> for ()
// where
//     P: ModelPath,
// {
//     fn prefix(path: P) -> Self {
//         ()
//     }
// }

impl ModelTrait for String {
    type Path = ();

    fn value(&self, _path: ()) -> AttrValue {
        self.clone().into()
    }

    fn set_value(&mut self, _path: (), value: &str) -> Result<(), String> {
        *self = value.to_owned();
        Ok(())
    }
}

impl ModelTrait for u32 {
    type Path = ();

    fn value(&self, _path: ()) -> AttrValue {
        self.to_string().into()
    }

    fn set_value(&mut self, _path: (), value: &str) -> Result<(), String> {
        match value.parse() {
            Ok(value) => {
                *self = value;
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
}

// pub trait FormTrait<M: ModelTrait> {
//     fn value(&self, path: M::Path) -> AttrValue;
//     fn set_value<S: ToString>(&mut self, path: M::Path, value: S);
// }

// pub struct Form<M: ModelTrait> {
//     model: M,
//     fields: HashMap<M::Path, Field>,
// }

// impl<M: ModelTrait> FormTrait<M> for Form<M> {
//     fn value(&self, &str) -> AttrValue {
//         self.model.value(path)
//     }

//     fn set_value<S: ToString>(&mut self, path: <M as ModelTrait>::Path, value: S) {
//         todo!()
//     }
// }
//

fn prefix_path<P: ModelTrait, C: ModelTrait>(prefix: &str, path: &str) -> String {
    format!("{prefix}.{path}")
}

pub trait FormTrait<T: ModelTrait> {
    fn value(&self, path: T::Path) -> AttrValue;
}

// pub enum Form<T: ModelTrait> {
//     Owned(FormOwned<T>),
//     Ref(FormOwned<T>, T::Path),
// }

// impl<T: ModelTrait> Form<T> {
//     pub fn new(model: T) -> Self {
//         Form::Owned(FormOwned::new(model))
//     }

//     pub fn form_ref(&self, path: &'static str) -> Self {
//         debug_assert!(!path.is_empty());

//         match self {
//             Form::Owned(owned) => Form::Ref(owned.clone(), path.to_owned()),
//             Form::Ref(owned, prefix) => Form::Ref(owned.clone(), prefix_path(prefix, path)),
//         }
//     }

//     pub fn value(&self, path: &'static str) -> AttrValue {
//         match self {
//             Form::Owned(owned) => owned.value(<_ as IntoField>::into_field(path)),
//             Form::Ref(owned, prefix) => owned.value(P::prefix(prefix, path)),
//         }
//     }

//     pub fn set_value(&self, path: T::Path, value: impl Into<AttrValue>) {
//         match self {
//             Form::Owned(owned) => owned.set_value(path, value),
//             Form::Ref(owned, prefix) => todo!(),
//         }
//     }

//     pub fn model(&self) -> Ref<T> {
//         match self {
//             Form::Owned(owned) | Form::Ref(owned, _) => owned.model(),
//         }
//     }
// }

pub struct FormOwned<T: ModelTrait> {
    model: Rc<RefCell<T>>,
    fields: Rc<RefCell<HashMap<T::Path, Field>>>,
}

impl<T: ModelTrait> FormOwned<T> {
    fn new(model: T) -> Self {
        let paths = model.paths();
        let fields = paths
            .into_iter()
            .map(|path| {
                let value = model.value(path);
                (
                    path,
                    Field {
                        value,
                        dirty: false,
                        valid: true,
                    },
                )
            })
            .collect();

        FormOwned {
            model: Rc::new(RefCell::new(model)),
            fields: Rc::new(RefCell::new(fields)),
        }
    }

    fn value(&self, path: T::Path) -> AttrValue {
        self.fields
            .deref()
            .borrow()
            .get(&path)
            .expect("Path is not part of the model")
            .value
            .clone()
    }

    fn set_value(&self, path: T::Path, value: impl Into<AttrValue>) {
        self.fields
            .borrow_mut()
            .get_mut(&path)
            .expect("Path is not part of the model")
            .value = value.into();
    }

    fn model(&self) -> Ref<T> {
        self.model.deref().borrow()
    }
}

impl<T: ModelTrait> Clone for FormOwned<T> {
    fn clone(&self) -> Self {
        Self {
            model: self.model.clone(),
            fields: self.fields.clone(),
        }
    }
}

// parent.child.grand_child.name
// Child(GrandChild(Root))

pub struct FormRef<P: ModelTrait, T: ModelTrait> {
    owned: FormOwned<P>,
    prefix: P::Path,
    _phantom: PhantomData<T>,
}

// impl<P: ModelTrait, T: ModelTrait> FormTrait<T> for FormRef<P, T> {
//     fn value(&self, path: T::Path) -> AttrValue {
//         // let prefixed_path = self.prefix.prefix(path);
//         // self.owned.value(prefixed_path)
//     }
// }

// pub struct FormRef<T: ModelTrait> {
//     parent: FormOwned<T>,
//     prefix: String,
// }

// impl<T: ModelTrait> Form<T> for FormRef<T> {
//     fn value(&self, path: &str) -> AttrValue {
//         self.parent.value(path)
//     }

//     fn form_ref(&self, path: &str) -> FormRef<T> {
//         FormRef {
//             parent: self.parent.clone(),
//             prefix: self
//                 .prefix
//                 .is_empty()
//                 .then(|| path.to_owned())
//                 .unwrap_or_else(|| format!("{}.{path}", self.prefix)),
//         }
//     }
// }

pub trait IntoField: Copy {
    fn into_field(path: &'static str) -> Self;
    fn root() -> Self;
    // fn prefix<T: IntoField>(self, path: T) -> Self;
    // fn replace_root(self, path: Self) -> Self;
}

impl IntoField for () {
    fn into_field(_path: &'static str) -> Self {
        debug_assert!(_path.is_empty());
        ()
    }

    fn root() -> Self {
        ()
    }

    // fn prefix<T: IntoField>(self, path: T) -> Self {
    //     panic!()
    // }

    // fn replace_root(self, path: Self) -> Self {
    //     path
    // }
}

pub trait ModelTrait {
    type Path: IntoField + std::hash::Hash + Eq + Copy;

    fn value(&self, field: Self::Path) -> AttrValue;
    fn set_value(&mut self, field: Self::Path, value: &str) -> Result<(), String>;
    fn paths(&self) -> Vec<Self::Path> {
        vec![]
    }
}

pub struct Field {
    value: AttrValue,
    dirty: bool,
    valid: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate as yew_form;
    use strum::EnumDiscriminants;
    use yew::html::IntoPropValue;
    use yew_form_derive::{field, Model};

    // #[test]
    // fn test_module_path() {
    //     #[derive(Debug)]
    //     enum ParentPath {
    //         Id,
    //         Child(ChildPath),
    //     }
    //     impl IntoField for ParentPath {
    //         fn into_field(path: &'static str) -> Self {
    //             println!("{path}");

    //             match path.split_once('.') {
    //                 None => match path {
    //                     "id" => Self::Id,
    //                     _ => panic!(),
    //                 },
    //                 Some((field, suffix)) => match field {
    //                     "child" => Self::Child(ChildPath::into_field(suffix)),
    //                     _ => panic!(),
    //                 },
    //             }
    //         }
    //     }

    //     impl PrefixPath<ChildPath> for ParentPath {
    //         fn prefix(path: ChildPath) -> Self {
    //             Self::Child(path)
    //         }
    //     }

    //     #[derive(Model)]
    //     struct Parent {
    //         id: u32,
    //         child: Child,
    //     }

    //     #[derive(Debug)]
    //     enum ChildPath {
    //         Name,
    //     }

    //     impl IntoField for ChildPath {
    //         fn into_field(path: &'static str) -> Self {
    //             match path {
    //                 "name" => Self::Name,
    //                 _ => panic!(),
    //             }
    //         }
    //     }

    //     macro_rules! field {
    //         ($($field:tt)+) => {
    //             <_ as IntoField>::into_field(stringify!($($field)+))
    //         };
    //     }
    //     #[derive(Model)]
    //     struct Child {
    //         name: String,
    //     }

    //     fn elide<T>(f: impl Fn() -> T) -> T {
    //         f()
    //     }

    //     fn get_field<T: IntoField>(path: &'static str) -> T {
    //         T::into_field(path)
    //     }

    //     let field: ParentPath = get_field("child.name");

    //     fn test_field(field: ParentPath) {
    //         println!("{field:?}")
    //     }

    //     test_field(get_field("child.name"));
    //     test_field(field!(child.name))
    //     // fn parse<T>() -> T {
    //     //     _::Id
    //     // }

    //     // let field: ParentPath = || {
    //     //     _::Id
    //     // }();

    //     // let path: ParentPath = field!(child.name);
    // }

    #[test]
    fn test_paths() {
        #[derive(Model)]
        struct Parent {
            id: u32,
            #[model]
            child: Child,
        }

        #[derive(Model)]
        struct Child {
            name: String,
            #[model]
            grand_child: GrandChild,
        }

        #[derive(Model)]
        struct GrandChild {
            name: String,
        }

        let parent = Parent {
            id: 1,
            child: Child {
                name: "child".into(),
                grand_child: GrandChild {
                    name: "grand_child".into(),
                },
            },
        };

        macro_rules! prefix {
            ($c:expr, $p:expr) => {
                (&($p, $c)).prefixer().prefix($c, $p)
            };
        }

        // fn prefix<C: IntoField, P: IntoField>(child: C, parent: P) -> P {
        //     prefix!(child, parent)
        // }

        struct PrefixAnyTag<P, C>(PhantomData<(P, C)>);
        struct PrefixTag<P, C>(PhantomData<(P, C)>);

        impl PrefixTag<ParentFields, ChildFields> {
            fn prefix(self, child: ChildFields, parent: ParentFields) -> ParentFields {
                println!("P P>C");
                match parent {
                    ParentFields::Child(ChildFields::Root) => ParentFields::Child(child),
                    _ => panic!(),
                }
            }
        }

        impl PrefixTag<ChildFields, GrandChildFields> {
            fn prefix(self, child: GrandChildFields, parent: ChildFields) -> ChildFields {
                println!("P C>G");
                match parent {
                    ChildFields::GrandChild(GrandChildFields::Root) => {
                        ChildFields::GrandChild(child)
                    }
                    _ => panic!(),
                }
            }
        }

        impl PrefixTag<GrandChildFields, GrandChildFields> {
            fn prefix(self, child: GrandChildFields, parent: GrandChildFields) -> GrandChildFields {
                println!("P G>G");
                child
                // match parent {
                //     ChildFields::GrandChild(GrandChildFields::Root) => {
                //         ChildFields::GrandChild(child)
                //     }
                //     _ => panic!(),
                // }
            }
        }

        fn prefix<C: IntoField>(child: C, parent: GrandChildFields) -> GrandChildFields {
            use Prefix;
            println!("{}", std::any::type_name::<C>());
            println!("{}", std::any::type_name::<GrandChildFields>());
            //(&&(parent, child)).prefixer().prefix(child, parent)
            prefix!(child, parent)
        }
        impl<T: IntoField> PrefixAnyTag<ChildFields, T> {
            fn prefix(self, child: T, parent: ChildFields) -> ChildFields {
                println!("P C>Any");

                println!("{}", std::any::type_name::<T>());
                //                prefix(child, parent)
                match parent {
                    ChildFields::Root => todo!(),
                    ChildFields::Name => todo!(),
                    // ChildFields::GrandChild(GrandChildFields::Root) => {
                    //     println!("here");
                    //     ChildFields::GrandChild(
                    //         (GrandChildFields::Root, child)
                    //             .prefixer()
                    //             .prefix(child, GrandChildFields::Root),
                    //     )
                    // }
                    ChildFields::GrandChild(model) => {
                        ChildFields::GrandChild(prefix!(child, model))
                    }
                }
            }
        }
        // impl PrefixAnyTag<ChildFields, GrandChildFields> {
        //     fn prefix(self, child: GrandChildFields, parent: ChildFields) -> ChildFields {
        //         println!("P C>Any");

        //         match parent {
        //             ChildFields::Root => todo!(),
        //             ChildFields::Name => todo!(),
        //             ChildFields::GrandChild(GrandChildFields::Root) => {
        //                 ChildFields::GrandChild(prefix!(child, GrandChildFields::Root))
        //             }
        //             ChildFields::GrandChild(model) => {
        //                 ChildFields::GrandChild(prefix!(child, model))
        //             }
        //         }
        //     }
        // }

        impl<T: IntoField> PrefixAnyTag<GrandChildFields, T> {
            fn prefix(self, child: T, parent: GrandChildFields) -> GrandChildFields {
                println!("P Any>G");
                println!("{}", std::any::type_name::<T>());
                match parent {
                    GrandChildFields::Root => prefix(child, GrandChildFields::Root),
                    GrandChildFields::Name => todo!(),
                }
            }
        }

        impl<T: IntoField> PrefixAnyTag<ParentFields, T> {
            fn prefix(self, child: T, parent: ParentFields) -> ParentFields
// where
            //     &'static (ParentFields, T): PrefixAny<ParentFields>,
            {
                println!("P P>Any");

                println!("{}", std::any::type_name::<T>());
                match parent {
                    ParentFields::Root => todo!(),
                    ParentFields::Id => todo!(),
                    ParentFields::Child(model) => ParentFields::Child(prefix!(child, model)),
                }
            }
        }

        // impl<T: IntoField> PrefixAnyTag<&T> {
        //     fn prefix<C: IntoField>(self, child: C, parent: &T) -> T {
        //         println!("P Any>Any");
        //         todo!()
        //     }
        // }

        trait Prefix<P, C> {
            fn prefixer(&self) -> PrefixTag<P, C> {
                PrefixTag::<P, C>(Default::default())
            }
        }

        trait PrefixAny<P, C> {
            fn prefixer(&self) -> PrefixAnyTag<P, C> {
                PrefixAnyTag::<P, C>(Default::default())
            }
        }

        impl Prefix<ParentFields, ChildFields> for (ParentFields, ChildFields) {}
        impl Prefix<ChildFields, GrandChildFields> for (ChildFields, GrandChildFields) {}
        impl Prefix<GrandChildFields, GrandChildFields> for (GrandChildFields, GrandChildFields) {}
        impl<P: IntoField, C: IntoField> PrefixAny<P, C> for &(P, C) {}
        // trait Suffix {
        //     type R;
        //     fn prefix(&self, path: Self::C) -> Self::R;
        // }

        // trait SuffixAny<T> {
        //     type R;
        //     fn prefix(&self, path: T) -> Self::R;
        // }

        // impl Suffix for ParentFields {
        //     type R = T;
        //     fn prefix(&self, path: ChildFields) -> Self::R {
        //         prefix!(*self, path)
        //     }
        // }
        // impl<T: IntoField> SuffixAny<ChildFields> for &T {
        //     type R = ParentFields;
        //     fn prefix(&self, path: ChildFields) -> Self::R {
        //         match self {
        //             ParentFields::Child(_) => ParentFields::Child(path),
        //             _ => panic!(),
        //         }
        //     }
        // }

        // impl<T: IntoField> Suffix<GrandChildFields> for T {
        //     type R = T;
        //     fn prefix(&self, path: GrandChildFields) -> Self::R {
        //         prefix!(*self, path)
        //     }
        // }
        // impl Suffix<GrandChildFields> for &ChildFields {
        //     type R = ChildFields;
        //     fn prefix(&self, path: GrandChildFields) -> Self::R {
        //         match self {
        //             ChildFields::GrandChild(_) => ChildFields::GrandChild(path),
        //             _ => panic!(),
        //         }
        //     }
        // }

        // impl<T: IntoField> SuffixAny<ParentFields> for T {
        //     fn prefix(&self, path: ParentFields) -> Self {
        //         println!("Parent &T");
        //         prefix!(self, path)
        //         // ParentFields::Root
        //         // match path {
        //         //     ParentFields::Child(model) => ParentFields::Child(prefix!(self, model)),
        //         //     _ => panic!(),
        //         // }
        //     }
        // }
        // impl Suffix<ParentFields> for ChildFields {
        //     fn prefix(&self, path: ParentFields) -> ChildFields {
        //         println!("Parent ChildFields");
        //         match path {
        //             ParentFields::Child(model) => ParentFields::Child(*self),
        //             _ => panic!(),
        //         }
        //     }
        // }

        // impl<T: IntoField> SuffixAny<ChildFields> for T {
        //     fn prefix(&self, path: &ChildFields) -> ChildFields {
        //         println!("Child &T");
        //         match path {
        //             ChildFields::Root => prefix!(*self, *path),
        //             ChildFields::GrandChild(model) => {
        //                 ChildFields::GrandChild(prefix!(*self, model))
        //             }
        //             _ => panic!("{path:?}"),
        //         }
        //     }
        // }
        // impl Suffix<ChildFields> for GrandChildFields {
        //     fn prefix(&self, path: ChildFields) -> ChildFields {
        //         println!("Child GrandChildFields");
        //         match path {
        //             ChildFields::GrandChild(model) => ChildFields::GrandChild(*self),
        //             _ => panic!(),
        //         }
        //     }
        // }
        // impl<T: IntoField> SuffixAny<GrandChildFields> for T {
        //     fn prefix(&self, path: &GrandChildFields) -> GrandChildFields {
        //         println!("GrandChild &T");
        //         GrandChildFields::Root
        //     }
        // }
        // impl IntoField for ParentFields {
        //     fn root() -> Self {
        //         todo!()
        //     }

        //     fn into_field(path: &'static str) -> Self {
        //         todo!()
        //     }

        //     fn with_prefix(self, path: &'static str) -> Self {
        //         match self {
        //             ParentFields::Child(ChildFields::Root) => ParentFields::Child(ChildFields::into_field(path)),
        //             ParentFields::Child(c) => c.with_prefix(path),
        //             _ => panic!(),
        //         }
        //     }
        // }
        //

        assert_eq!(
            parent.child.paths(),
            vec![
                ChildFields::Name,
                ChildFields::GrandChild(GrandChildFields::Name)
            ]
        );
        assert_eq!(
            parent.paths(),
            vec![
                ParentFields::Id,
                ParentFields::Child(ChildFields::Name),
                ParentFields::Child(ChildFields::GrandChild(GrandChildFields::Name))
            ]
        );
        println!("first");
        assert_eq!(
            prefix!(
                ChildFields::into_field("grand_child.name"),
                ParentFields::Child(ChildFields::Root)
            ),
            ParentFields::Child(ChildFields::GrandChild(GrandChildFields::Name))
        );

        println!("first");
        assert_eq!(
            prefix!(
                GrandChildFields::into_field("name"),
                ChildFields::GrandChild(GrandChildFields::Root)
            ),
            ChildFields::GrandChild(GrandChildFields::Name)
        );

        println!("second");
        assert_eq!(
            prefix!(
                GrandChildFields::into_field("name"),
                ParentFields::Child(ChildFields::GrandChild(GrandChildFields::Root))
            ),
            ParentFields::Child(ChildFields::GrandChild(GrandChildFields::Name))
        );
        // assert_eq!(
        //     ParentFields::Child(ChildFields::Name)
        //         .prefix(ChildFields::into_field("grand_child.name")),
        //     ParentFields::Child(ChildFields::GrandChild(GrandChildFields::Name))
        // );
    }
}
