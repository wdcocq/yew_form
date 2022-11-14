use web_sys::HtmlInputElement;
use web_sys::InputEvent;
use yew::prelude::*;

use crate::form::Form;
use crate::Model;

#[derive(Properties, PartialEq, Clone)]
pub struct FieldProps<T: Model> {
    #[prop_or_default]
    pub autocomplete: bool,
    #[prop_or_else(|| "text".into())]
    pub input_type: AttrValue,
    pub field_name: AttrValue,
    pub form: Form<T>,
    #[prop_or_default]
    pub placeholder: AttrValue,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_else(|| "form-control".into())]
    pub classes: Classes,
    #[prop_or_else(|| "is-invalid".into())]
    pub classes_invalid: Classes,
    #[prop_or_else(|| "is-valid".into())]
    pub classes_valid: Classes,
    #[prop_or_default]
    pub oninput: Callback<InputEvent>,
}

#[function_component(Field)]
pub fn field<T: Model>(
    FieldProps {
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
    }: &FieldProps<T>,
) -> Html {
    let field = form.field(field_name);
    let classes = classes!(
        classes.clone(),
        field.dirty.then(|| match field.valid {
            true => classes_valid.clone(),
            false => classes_invalid.clone(),
        })
    );

    let autocomplete = if *autocomplete { "on" } else { "off" };
    let oninput = {
        let form = form.clone();
        let field_name = field_name.clone();

        oninput.reform(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                form.field_mut(&field_name).set_value(input.value());
            }
            e
        })
    };

    html! {
        <input
            id={field_name}
            class={classes}
            type={input_type}
            {autocomplete}
            {placeholder}
            value={field.value()}
            {oninput}
            disabled={*disabled}
        />
    }
}
