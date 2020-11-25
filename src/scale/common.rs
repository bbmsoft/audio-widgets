use scales::prelude::*;

pub type ScaleValue = f64;

#[derive(Debug, PartialEq, Clone)]
pub struct ScaleModel<S>
where
    S: Scale<f64>,
{
    pub scale: S,
}

impl<S> ScaleModel<S>
where
    S: Scale<f64>,
{
    pub fn new(scale: S) -> ScaleModel<S> {
        ScaleModel { scale }
    }
}
