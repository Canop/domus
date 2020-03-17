
use {
    wasm_bindgen::{
        prelude::*,
        JsCast,
    },
    web_sys::{
        Element,
        Document,
        HtmlElement,
        console,
    },
};

pub trait DomusElement {
    fn empty(&self);
}

impl DomusElement for HtmlElement {
    /// only remove child elements, not text nodes
    fn empty(&self) {
        while let Some(child) = self.first_element_child() {
            child.remove();
        }
    }
}

pub fn doc() -> Document {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    document
}

// will crash your app when there's no body in the current document.
// This is usually fine.
pub fn body() -> HtmlElement {
    doc().body().unwrap()
}

pub fn js_err<T>(message: &str) -> Result<T, JsValue> {
    Err(JsValue::from_str(message))
}

pub fn by_id(id: &str) -> Option<HtmlElement> {
    doc()
        .get_element_by_id(id)
        .and_then(|element|
             element
            .dyn_into::<HtmlElement>()
            .ok()
        )
}

// TODO in order to eliminate dynamic errors we should use
// an enum of tags instead of taking a str
pub fn tag(tag_name: &str) -> Result<HtmlElement, JsValue> {
    let e: Element = doc().create_element(tag_name)?;
    match e.dyn_into::<HtmlElement>() {
        Ok(e) => Ok(e),
        Err(_) => js_err(&format!("{:?} tag not making a HtmlElement", tag_name)),
    }
}

pub fn tag_class(tag_name: &str, class: &str) -> Result<HtmlElement, JsValue> {
    let e = tag(tag_name)?;
    e.class_list().add_1(class)?;
    Ok(e)
}

pub fn remove_by_selector(selector: &str) {
    let node_list = doc().query_selector_all(selector).unwrap();
    for i in 0..node_list.length() {
        let node = node_list.item(i);
        if let Some(node) = node {
            if let Ok(e) = node.dyn_into::<HtmlElement>() {
                e.remove();
            }
        }
    }
}

pub fn log_str(s: &str) {
    console::log_1(&JsValue::from_str(s));
}

/// log anything. Uses format!
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => (crate::domus::log_str(&format!($($arg)*)));
}

