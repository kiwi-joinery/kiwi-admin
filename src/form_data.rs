use wasm_bindgen::JsValue;
use web_sys::{FormData, HtmlFormElement};
use yew::html::Scope;
use yew::prelude::*;

pub trait GetFormData<COMP: Component> {
    fn on_form_submit<F, M>(&self, function: F) -> Callback<FocusEvent>
    where
        M: Into<COMP::Message>,
        F: Fn(FormData) -> M + 'static;
}

impl<COMP: Component> GetFormData<COMP> for Scope<COMP> {
    fn on_form_submit<F, M>(&self, function: F) -> Callback<FocusEvent>
    where
        M: Into<COMP::Message>,
        F: Fn(FormData) -> M + 'static,
    {
        self.callback(move |e: FocusEvent| {
            e.prevent_default();
            let f: HtmlFormElement = JsValue::from(e.target().unwrap()).into();
            let fd = FormData::new_with_form(&f).unwrap();
            function(fd)
        })
    }
}
