use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
mod function;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Type alias for the result of the plotter.
pub type DrawResult<T> = Result<T, Box<dyn std::error::Error>>;

#[wasm_bindgen]
pub struct Chart {
    convert: Box<dyn Fn((i32, i32)) -> Option<(f64, f64)>>,
}

#[wasm_bindgen]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl Chart {
    /// Draw function on the canvas element by id.
    pub fn cobb_douglas(canvas_id: &str, exponent: f32) -> Result<Chart, JsValue> {
        let map_coord = function::draw_function(canvas_id, exponent).map_err(|err| err.to_string())?;
        Ok(Chart {
            convert: Box::new(move |coord| map_coord(coord).map(|(x, y)| (x.into(), y.into()))),
        })
    }
}