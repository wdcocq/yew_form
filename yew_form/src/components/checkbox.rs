use crate::{Form, Model};
use yew::prelude::*;

pub enum CheckBoxMessage {
    OnToggle,
}

#[derive(Properties, PartialEq, Clone)]
pub struct CheckBoxProps<T: Model> {
    pub field_name: AttrValue,
    pub form: Form<T>,
    #[prop_or_else(|| "form-check-input, form-input".into())]
    pub classes: Classes,
    #[prop_or_default]
    pub ontoggle: Callback<bool>,
}

#[function_component(CheckBox)]
pub fn check_box<T: Model>(
    CheckBoxProps {
        field_name,
        form,
        classes,
        ontoggle,
    }: &CheckBoxProps<T>,
) -> Html {
    let field = form.field(field_name);
    let field_value = field.value() == "true";

    let ontoggle = {
        let form = form.clone();
        let field_value = field_value.clone();
        let field_name = field_name.clone();

        ontoggle.reform(move |_| {
            let value = !field_value;
            let mut field = form.field_mut(&field_name);
            field.set_value(value.to_string());
            value
        })
    };

    html! {
        <input
            class={classes.clone()}
            type="checkbox"
            value={ field_name }
            onclick={ ontoggle }
            checked={ field_value }
            class={classes.clone()}
         />
    }
}
