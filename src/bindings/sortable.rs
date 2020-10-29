use js_sys::Object;
use wasm_bindgen::prelude::*;
use web_sys::Element;

#[wasm_bindgen]
extern "C" {
    pub type OnEndEvent;

    #[wasm_bindgen(method, getter=oldIndex, structural)]
    pub fn old_index(this: &OnEndEvent) -> u32;

    #[wasm_bindgen(method, getter=newIndex, structural)]
    pub fn new_index(this: &OnEndEvent) -> u32;
}

#[wasm_bindgen]
extern "C" {
    pub type SortableOptions;
    #[wasm_bindgen(method, setter=onEnd, structural)]
    pub fn set_on_end(this: &SortableOptions, val: &Closure<dyn FnMut(OnEndEvent)>);
}

#[wasm_bindgen]
extern "C" {
    pub type Sortable;

    #[wasm_bindgen(constructor)]
    pub fn create(element: &Element, options: SortableOptions) -> Sortable;
}

impl SortableOptions {
    pub fn new() -> Self {
        Self {
            obj: Object::new().into(),
        }
    }
}
