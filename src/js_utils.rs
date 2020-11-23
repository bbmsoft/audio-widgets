use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

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

pub fn set_style(element: &HtmlElement, key: &str, value: &str) {
    // TODO handle errors
    element.style().set_property(key, value).unwrap();
}

pub fn register_global_mouse_move_listener(listener: Rc<Closure<dyn Fn(MouseEvent) -> ()>>) {
    let doc = document();

    let on_up: Rc<Cell<Option<Closure<dyn Fn(MouseEvent) -> ()>>>> = Rc::new(Cell::new(None));

    let listener_cb = listener.clone();
    let on_up_cb = on_up.clone();

    let on_up_callback = Closure::wrap(Box::new(move |_| {
        unregister_global_listener("mousemove", &listener_cb);
        if let Some(cl) = on_up_cb.take() {
            unregister_global_listener("mouseup", &cl);
        }
    }) as Box<dyn Fn(MouseEvent) -> ()>);

    doc.add_event_listener_with_callback("mouseup", on_up_callback.as_ref().unchecked_ref())
        .expect("registering listener failed");
    doc.add_event_listener_with_callback("mousemove", (*listener).as_ref().unchecked_ref())
        .expect("registering listener failed");

    on_up.set(Some(on_up_callback));
}

fn unregister_global_listener(event_type: &str, listener: &Closure<dyn Fn(MouseEvent) -> ()>) {
    let doc = document();
    doc.remove_event_listener_with_callback(event_type, listener.as_ref().unchecked_ref())
        .expect("registering listener failed");
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> web_sys::Document {
    window().document().expect("no global `document` exists")
}
