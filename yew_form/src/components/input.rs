use strum::IntoStaticStr;
use web_sys::HtmlInputElement;
use web_sys::InputEvent;
use yew::html::ImplicitClone;
use yew::html::IntoPropValue;
use yew::prelude::*;

#[cfg(feature = "ybc")]
use ybc;

use crate::form::Form;
use crate::Model;

#[derive(Debug, Copy, Clone, PartialEq, Eq, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum InputType {
    Text,
    Password,
    Email,
    Tel,
    Url,
    Date,
}

impl ImplicitClone for InputType {}

impl IntoPropValue<Option<AttrValue>> for InputType {
    fn into_prop_value(self) -> Option<AttrValue> {
        <AttrValue as From<&'static str>>::from(self.into()).into()
    }
}

#[cfg(feature = "ybc")]
impl InputType {
    fn ybc_type(&self) -> Option<ybc::InputType> {
        match self {
            InputType::Text => Some(ybc::InputType::Text),
            InputType::Password => Some(ybc::InputType::Password),
            InputType::Email => Some(ybc::InputType::Email),
            InputType::Tel => Some(ybc::InputType::Tel),
            _ => None,
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct InputProps<T: Model> {
    #[prop_or_default]
    pub autocomplete: bool,
    #[prop_or(InputType::Text)]
    pub input_type: InputType,
    pub field_name: AttrValue,
    pub form: Form<T>,
    #[prop_or_default]
    pub placeholder: AttrValue,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub classes_invalid: Classes,
    #[prop_or_default]
    pub classes_valid: Classes,
    #[prop_or_default]
    pub oninput: Callback<InputEvent>,
}

#[function_component(Input)]
pub fn input<T: Model>(
    InputProps {
        autocomplete,
        input_type,
        field_name,
        form,
        placeholder,
        disabled,
        classes,
        classes_invalid,
        classes_valid,
        oninput,
    }: &InputProps<T>,
) -> Html {
    let field = form.field(field_name);
    let classes = classes!(
        classes.clone(),
        field.dirty.then(|| match field.valid {
            true => classes_valid.clone(),
            false => classes_invalid.clone(),
        })
    );

    let oninput = {
        let form = form.clone();
        let field_name = field_name.clone();

        oninput.reform(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                form.set_value(&field_name, input.value());
            }
            e
        })
    };

    // If a valid bulma/ybc input class return early with ybc element
    #[cfg(feature = "ybc")]
    if let Some(input_type) = input_type.ybc_type() {
        return html! {
            <ybc::Input
                name={field_name}
                {classes}
                r#type={input_type}
                autocomplete={*autocomplete}
                {placeholder}
                value={&field.value}
                update={oninput}
                disabled={*disabled}
            />
        };
    }

    // If not one of the valid bulma/ybc input classes, still add the 'input' class so it's styled properly.
    #[cfg(feature = "ybc")]
    let classes = classes!(classes, "input");

    let autocomplete = if *autocomplete { "on" } else { "off" };

    html! {
        <input
            id={field_name}
            class={classes}
            type={*input_type}
            {autocomplete}
            {placeholder}
            value={&field.value}
            {oninput}
            disabled={*disabled}
        />
    }
}
