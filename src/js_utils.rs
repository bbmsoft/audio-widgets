use crate::*;
use utils::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

pub fn get_style(
    style_name: impl AsRef<str>,
    style: &Option<CssStyleDeclaration>,
    default: Option<&str>,
) -> Option<String> {
    let prop = style
        .as_ref()?
        .get_property_value(style_name.as_ref())
        .ok()
        .and_then(|v| if v.is_empty() { None } else { Some(v) });
    prop.or(default.map(|s| s.to_owned()))
}

pub fn set_fill(context: &CanvasRenderingContext2d, style: Option<impl AsRef<str>>) {
    if let Some(style) = style {
        context.set_fill_style(&style.as_ref().into());
    }
}

pub fn set_stroke(context: &CanvasRenderingContext2d, style: Option<impl AsRef<str>>) {
    if let Some(style) = style {
        context.set_stroke_style(&style.as_ref().into());
    }
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn get_styles(element: &Element) -> Option<CssStyleDeclaration> {
    web_sys::window().and_then(|w| {
        w.get_computed_style(&element)
            .expect("error getting element style")
    })
}

pub fn get_context_2d(canvas: &HtmlCanvasElement) -> Option<CanvasRenderingContext2d> {
    if let Ok(Some(ctx)) = canvas.get_context("2d") {
        if let Ok(ctx) = ctx.dyn_into::<web_sys::CanvasRenderingContext2d>() {
            Some(ctx)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn set_style(element: &HtmlElement, key: &str, value: &str) {
    element.style().set_property(key, value).ignore();
}

pub fn register_global_listener(event_type: &str, listener: &Closure<dyn Fn(MouseEvent) -> ()>) {
    let doc = document();
    doc.add_event_listener_with_callback(event_type, listener.as_ref().unchecked_ref())
        .expect("registering listener failed");
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> web_sys::Document {
    window().document().expect("no global `document` exists")
}

impl<DR: AsRef<DomRect>> From<DR> for Bounds {
    fn from(dr: DR) -> Self {
        let dr = dr.as_ref();
        Bounds {
            x: dr.x(),
            y: dr.y(),
            width: dr.width(),
            height: dr.height(),
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(a: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (#[allow(unused_unsafe)]unsafe{log(&format_args!($($t)*).to_string())})
}
