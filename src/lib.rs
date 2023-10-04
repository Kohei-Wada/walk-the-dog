use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use rand::prelude::*;

fn draw_triangle(context: &web_sys::CanvasRenderingContext2d, 
                 points: [(f64, f64); 3],
                 color: (u8, u8, u8)) {

    let color_str = format!("rgb({}, {}, {})", color.0, color.1, color.2);
    context.set_fill_style(&wasm_bindgen::JsValue::from_str(&color_str));

    let [top, left, right] = points;
    context.move_to(top.0, top.1);
    context.begin_path();
    context.line_to(left.0, left.1);
    context.line_to(right.0, right.1);
    context.line_to(top.0, top.1);
    context.close_path();
    context.stroke();
    context.fill();
}

fn sierpinsk(context: &web_sys::CanvasRenderingContext2d, 
             points: [(f64, f64); 3], 
             color: (u8, u8, u8),
             depth: u8) {
    draw_triangle(context, points, color);

    let depth = depth - 1;
    let [top, left, right] = points;
    if depth > 0 {

        let mut rng = thread_rng();
        let next_color = (
            rng.gen_range(0..255),
            rng.gen_range(0..255),
            rng.gen_range(0..255),
        );

        let left_mid = ((top.0 + left.0) / 2.0, (top.1 + left.1) / 2.0);
        let right_mid = ((top.0 + right.0) / 2.0, (top.1 + right.1) / 2.0);
        let bottom_mid = ((left.0 + right.0) / 2.0, (left.1 + right.1) / 2.0);
        sierpinsk(&context, [top, left_mid, right_mid], next_color, depth);
        sierpinsk(&context, [left_mid, left, bottom_mid], next_color, depth);
        sierpinsk(&context, [right_mid, bottom_mid, right], next_color, depth);
    }
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();
    let document = window.document().expect("should have a document on window");
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    sierpinsk(&context, [(300.0, 0.0), (0.0, 600.0), (600.0, 600.0)], (0, 255, 0),  5);
    Ok(())
}