use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use web_sys::CssStyleDeclaration;
use web_sys::Element;
use web_sys::HtmlCanvasElement;

pub fn get_style(
    style_name: impl AsRef<str>,
    style: &Option<CssStyleDeclaration>,
    default: Option<&str>,
) -> Option<String> {
    if let Some(style) = style.as_ref() {
        if let Ok(style) = style.get_property_value(style_name.as_ref()) {
            if !style.is_empty() {
                Some(style)
            } else {
                default.map(|s| s.to_owned())
            }
        } else {
            default.map(|s| s.to_owned())
        }
    } else {
        default.map(|s| s.to_owned())
    }
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

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}
