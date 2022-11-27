use web_sys::HtmlTextAreaElement;
use web_sys::InputEvent;

use yew::html::ImplicitClone;
use yew::html::IntoPropValue;
use yew::prelude::*;

use crate::form::Form;
use crate::Model;

#[derive(Clone, Copy, PartialEq)]
pub enum Wrap {
    Soft,
    Hard,
}

impl ImplicitClone for Wrap {}

impl IntoPropValue<Option<AttrValue>> for &Wrap {
    fn into_prop_value(self) -> Option<AttrValue> {
        Some(match self {
            Wrap::Soft => "soft".into(),
            Wrap::Hard => "hard".into(),
        })
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct TextAreaProps<T: Model> {
    pub form: Form<T>,
    pub field_name: AttrValue,
    pub oninput: Callback<InputEvent>,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub classes_invalid: Classes,
    #[prop_or_default]
    pub classes_valid: Classes,
    #[prop_or(20)]
    pub cols: u32,
    #[prop_or(5)]
    pub rows: u32,
    #[prop_or_default]
    pub placeholder: AttrValue,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or(Wrap::Soft)]
    pub wrap: Wrap,
    #[prop_or_default]
    pub spellcheck: bool,
    #[prop_or_default]
    pub autocomplete: bool,
    #[prop_or_default]
    pub autocorrect: bool,
}

#[function_component(TextArea)]
pub fn text_area<T: Model>(
    TextAreaProps {
        form,
        field_name,
        oninput,
        classes,
        classes_invalid,
        classes_valid,
        cols,
        rows,
        placeholder,
        disabled,
        wrap,
        spellcheck,
        autocomplete,
        autocorrect,
    }: &TextAreaProps<T>,
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
            if let Some(input) = e.target_dyn_into::<HtmlTextAreaElement>() {
                form.set_value(&field_name, input.value());
            }
            e
        })
    };

    html! {
        <textarea
            id={field_name}
            class={classes}
            name={field_name}
            cols={cols.to_string()}
            rows={rows.to_string()}
            {placeholder}
            {wrap}
            spellcheck={spellcheck.to_string()}
            autocomplete={autocomplete.to_string()}
            autocorrect={autocorrect.to_string()}
            {oninput}
            disabled={*disabled}
        />
    }
}
