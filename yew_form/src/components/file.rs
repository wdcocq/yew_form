use crate::form::Form;
use crate::Model;
use web_sys::HtmlInputElement;
use web_sys::InputEvent;
use yew::prelude::*;

pub enum FileMessage {
    OnInput(InputEvent),
}

#[derive(Properties, PartialEq, Clone)]
pub struct FilePropeties<T: Model> {
    pub form: Form<T>,
    pub field_name: AttrValue,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub multiple: bool,
    #[prop_or_default]
    pub accept: AttrValue,
    #[prop_or_default]
    pub capture: AttrValue,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_else(|| "is-invalid".into())]
    pub classes_invalid: Classes,
    #[prop_or_else(|| "is-valid".into())]
    pub classes_valid: Classes,
    #[prop_or_default]
    pub oninput: Callback<InputEvent>,
}

#[function_component(File)]
pub fn file<T: Model>(
    FilePropeties {
        form,
        field_name,
        disabled,
        multiple,
        accept,
        capture,
        classes,
        classes_valid,
        classes_invalid,
        oninput,
    }: &FilePropeties<T>,
) -> Html {
    let field = form.field(field_name);
    let classes = classes!(
        classes.clone(),
        field.dirty.then(|| match field.valid {
            true => classes_valid.clone(),
            false => classes_invalid.clone(),
        })
    );
    let oninput = oninput.reform({
        let form = form.clone();
        let field_name = field_name.clone();
        move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                form.field_mut(&field_name).set_value(input.value());
            }

            e
        }
    });

    html! {
        <input
            id={field_name}
            type="file"
            name={field_name}
            {accept}
            disabled={*disabled}
            multiple={*multiple}
            class={classes}
            {oninput}
        />
    }
}
