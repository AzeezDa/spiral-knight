mod spiral;
mod utils;

use crate::spiral::{place_knights, SpiralGrid, MAX_KNIGHTS};
use lazy_static::lazy_static;
use rayon::prelude::*;
use std::sync::RwLock;
use wasm_bindgen::{prelude::*, Clamped};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlInputElement, ImageData};

// Caching the last spiral drawn
lazy_static! {
    static ref LAST_SPIRAL: RwLock<Option<(usize, Vec<(i32, i32)>, SpiralGrid)>> =
        RwLock::new(None);
}

const EMPTY_CELL_INDEX: usize = 0;

// Used for debug purposes
#[wasm_bindgen]
extern "C" {
    fn alert(message: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn draw(size: usize) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();

    // Get canvas to draw on
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

    let (knights, colours, _) = get_knight_colours_indices();
    let grid = get_grid(size, &knights);

    let height = grid.height();
    let width = height + 1;
    let mut backbuffer = vec![0u8; width * height * 4];

    canvas.set_width(width as u32);
    canvas.set_height(height as u32);

    // Draw the image on the buffer then update the canvas
    backbuffer
        .par_chunks_mut(4)
        .zip(grid.grid().par_iter())
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

fn get_grid(size: usize, knights: &[(i32, i32)]) -> SpiralGrid {
    let mut last_spiral = LAST_SPIRAL
        .write()
        .expect("Lock poisoned; should not happen");
    if let Some((last_size, last_knights, last_grid)) = last_spiral.as_ref() {
        if *last_size == size && &knights == last_knights {
            return last_grid.clone();
        }
    }
    let new_grid = place_knights(size, knights);
    *last_spiral = Some((size, knights.to_vec(), new_grid.clone()));

    new_grid
}

#[wasm_bindgen]
pub fn get_max_knights() -> u32 {
    MAX_KNIGHTS as u32
}

// knight == 0      : empty cells
// 1 <= knight <= 8 : cells occupied by knight of given index
#[wasm_bindgen]
pub fn get_sequence(size: usize, knight_or_empty: usize) -> String {
    let (knights, _, indices) = get_knight_colours_indices();
    let grid = get_grid(size, &knights);

    let index_corrected = if knight_or_empty == EMPTY_CELL_INDEX {
        EMPTY_CELL_INDEX
    } else {
        indices
            .iter()
            .position(|index| *index == knight_or_empty)
            .unwrap_or(usize::MAX - 1)
            + 1
    };

    itertools::Itertools::intersperse(
        grid.spiral_iterator()
            .enumerate()
            .filter_map(|(index, cell)| {
                if cell.occupied_by().unwrap_or(EMPTY_CELL_INDEX) == index_corrected {
                    Some(index)
                } else {
                    None
                }
            })
            .map(|index| index.to_string()),
        ",".to_string(),
    )
    .collect()
}

fn get_knight_colours_indices() -> (Vec<(i32, i32)>, Vec<(u8, u8, u8)>, Vec<usize>) {
    let document = web_sys::window().unwrap().document().unwrap();
    // Get activate (non (0, 0)) knights and their colours.
    let mut knights = vec![];
    let mut colours = vec![];
    let mut indices = vec![];
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
            indices.push(i as usize);
        }
    }

    (knights, colours, indices)
}
