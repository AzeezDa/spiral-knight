mod spiral;
mod utils;

use rayon::prelude::*;
use wasm_bindgen::{prelude::*, Clamped};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlInputElement, ImageData};

use crate::spiral::MAX_KNIGHTS;

#[wasm_bindgen]
extern "C" {
    fn alert(message: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn draw(size: usize) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id("output")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let mut knights = vec![];
    let mut colours = vec![];
    for i in 1..=MAX_KNIGHTS {
        let dx = document
            .get_element_by_id(&format!("Knight {i} dx"))
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value_as_number() as i32;

        let dy = document
            .get_element_by_id(&format!("Knight {i} dy"))
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value_as_number() as i32;

        let colour = document
            .get_element_by_id(&format!("Knight {i} colour"))
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value();

        if dx != 0 || dy != 0 {
            let r = u8::from_str_radix(&colour[1..=2], 16).unwrap();
            let g = u8::from_str_radix(&colour[3..=4], 16).unwrap();
            let b = u8::from_str_radix(&colour[5..=6], 16).unwrap();
            knights.push((dx, dy));
            colours.push((r, g, b));
        }
    }

    let spiral = spiral::place_knights(size, &knights);
    let width = size + 1;
    let height = size;

    canvas.set_width(width as u32);
    canvas.set_height(height as u32);

    let mut backbuffer = vec![0u8; width * height * 4];

    backbuffer
        .par_chunks_mut(4)
        .zip(spiral.grid().par_iter())
        .for_each(|(buffer, cell)| {
            let (r, g, b) = cell.to_colour(&colours);
            buffer[0] = r;
            buffer[1] = g;
            buffer[2] = b;
            buffer[3] = 255;
        });

    let image_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&backbuffer), // Wrap the slice with Clamped
        width as u32,
        height as u32,
    )?;

    context.put_image_data(&image_data, 0., 0.)?;

    Ok(())
}

#[wasm_bindgen]
pub fn get_max_knights() -> u32 {
    MAX_KNIGHTS as u32
}
