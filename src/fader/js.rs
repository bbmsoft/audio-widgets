use crate::*;
use web_sys::*;

#[derive(Debug, Clone)]
pub struct Elements {
    pub knob: HtmlElement,
    pub tooltip: Option<HtmlElement>,
    pub bounds: Bounds,
    pub knob_bounds: Bounds,
    pub pixel_scale: PixelScale,
    pub scale_pixel_scale: PixelScale,
}
