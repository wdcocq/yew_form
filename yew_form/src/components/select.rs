use std::rc::Rc;

use web_sys::HtmlSelectElement;
use web_sys::InputEvent;
use yew::html::ChildrenRenderer;
use yew::prelude::*;
use yew::virtual_dom::VChild;
use yew::virtual_dom::VText;

#[cfg(feature = "ybc")]
use ybc;

use crate::form::Form;
use crate::Model;

pub enum SelectMessage {
    OnInput(InputEvent),
}

#[derive(Clone, PartialEq)]
pub enum Options {
    Controlled(VChild<SelectOption>),
    Uncontrolled(Html),
}

impl From<VChild<SelectOption>> for Options {
    fn from(child: VChild<SelectOption>) -> Self {
        Options::Controlled(child)
    }
}

impl From<Html> for Options {
    fn from(child: Html) -> Self {
        Options::Uncontrolled(child)
    }
}

impl Into<Html> for Options {
    fn into(self) -> Html {
        match self {
            Options::Controlled(child) => child.into(),
            Options::Uncontrolled(child) => child.into(),
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct SelectProps<T: Model> {
    pub form: Form<T>,
    pub field_name: AttrValue,
    pub children: ChildrenRenderer<Options>,
    #[prop_or_default]
    pub autocomplete: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub multiple: bool,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub classes_valid: Classes,
    #[prop_or_default]
    pub classes_invalid: Classes,
    #[prop_or_default]
    pub onchange: Callback<Event>,
}

#[function_component(Select)]
pub fn select<T: Model>(
    SelectProps {
        form,
        field_name,
        autocomplete,
        disabled,
        multiple,
        classes,
        classes_valid,
        classes_invalid,
        children,
        onchange,
    }: &SelectProps<T>,
) -> Html {
    let field = form.field(&field_name);
    let selected = &field.value;
    let classes = classes!(
        classes.clone(),
        field.dirty.then(|| match field.valid {
            true => classes_valid.clone(),
            false => classes_invalid.clone(),
        })
    );

    let onchange = {
        let form = form.clone();
        let field_name = field_name.clone();

        onchange.reform(move |e: Event| {
            if let Some(input) = e.target_dyn_into::<HtmlSelectElement>() {
                form.set_value(&field_name, input.value());
            }

            e
        })
    };

    #[cfg(feature = "ybc")]
    html! {
       <ybc::Select
            name={field_name}
            {classes}
            disabled={*disabled}
            update={onchange}>
            { for children.iter().map(|option| {
                match option {
                    Options::Controlled(mut option) => {
                        let mut props = Rc::make_mut(&mut option.props);
                        props.selected = props.value == *selected;
                        option.into()
                    },
                    Options::Uncontrolled(option) => {
                        option
                    }
                }
            })}
       </ybc::Select>
    }

    #[cfg(not(feature = "ybc"))]
    html! {
        <select
            id={field_name}
            name={field_name}
            autocomplete={if *autocomplete {"on"} else {"off"}}
            disabled={*disabled}
            multiple={*multiple}
            class={classes}
            {onchange}
        >
            { for children.iter().map(|option| {
                match option {
                    Options::Controlled(mut option) => {
                        let mut props = Rc::make_mut(&mut option.props);
                        props.selected = props.value == *selected;
                        option.into()
                    },
                    Options::Uncontrolled(option) => {
                        option
                    }
                }
            })}
        </select>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct SelectOptionProps {
    pub value: AttrValue,
    #[prop_or_default]
    pub children: Option<ChildrenRenderer<VText>>,
    #[prop_or_default]
    selected: bool,
}

#[function_component(SelectOption)]
pub fn select_item(
    SelectOptionProps {
        value,
        children,
        selected,
    }: &SelectOptionProps,
) -> Html {
    html! {
        <option selected={*selected} {value}>
            if let Some(children) = children {
                {children.clone()}
            } else {
                {value}
            }
        </option>
    }
}
