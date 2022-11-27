use crate::{Form, Model};
use yew::prelude::*;

pub enum CheckBoxMessage {
    OnToggle,
}

#[derive(Properties, PartialEq, Clone)]
pub struct CheckBoxProps<T: Model> {
    pub field_name: AttrValue,
    pub form: Form<T>,
    #[prop_or_default]
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
    let value = form.value(field_name) == "true";

    let ontoggle = {
        let form = form.clone();
        let field_name = field_name.clone();

        ontoggle.reform(move |_| {
            let value = !value;
            form.set_value(&field_name, value.to_string());
            value
        })
    };

    html! {
        <input
            class={classes.clone()}
            type="checkbox"
            value={value.to_string()}
            onclick={ontoggle}
            checked={value}
            class={classes.clone()}
         />
    }
}
