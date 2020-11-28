use crate::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

pub fn get_style(
    style_name: impl AsRef<str>,
    style: &Option<CssStyleDeclaration>,
    default: Option<&str>,
) -> Option<String> {
    let prop = style.as_ref()?.get_property_value(style_name.as_ref()).ok();
    prop.or(default.map(|s| s.to_owned()))
}

pub fn get_font_size(style: &Option<CssStyleDeclaration>) -> Option<usize> {
    style
        .as_ref()?
        .get_property_value("font-size")
        .ok()?
        .split("px")
        .next()?
        .to_owned()
        .parse()
        .ok()
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

pub fn set_font(context: &CanvasRenderingContext2d, style: &str) {
    context.set_font(style);
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
    // TODO handle errors
    element.style().set_property(key, value).unwrap();
}

pub fn register_global_listener(event_type: &str, listener: &Closure<dyn Fn(MouseEvent) -> ()>) {
    let doc = document();
    doc.add_event_listener_with_callback(event_type, listener.as_ref().unchecked_ref())
        .expect("registering listener failed");
}

pub fn format_font(font_size: &Option<usize>, font_family: &Option<impl AsRef<str>>) -> String {
    let font_size: String = font_size
        .map(|v| format!("{}px", v))
        .unwrap_or_else(|| "11pt".to_owned());
    let font_family: &str = font_family
        .as_ref()
        .map(|v| v.as_ref())
        .unwrap_or("sans-serif");
    format!("{} {}", font_size, font_family)
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
