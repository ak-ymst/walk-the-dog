use rand::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
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

    console::log_1(&JsValue::from_str("Hello World"));

    sierpinki(
        &context,
        [(300.0, 0.0), (0.0, 600.0), (600.0, 600.0)],
        (0, 255, 0),
        5,
    );

    Ok(())
}

fn sierpinki(
    context: &web_sys::CanvasRenderingContext2d,
    points: [(f64, f64); 3],
    color: (u8, u8, u8),
    depth: u8,
) {
    draw_triangle(&context, points, color);

    let depth = depth - 1;

    let [top, left, right] = points;
    if depth > 0 {
        let next_color = get_color(depth);

        let left_mid = midpoint(top, left);
        let right_mid = midpoint(top, right);
        let bottom_mid = midpoint(left, right);
        sierpinki(&context, [top, left_mid, right_mid], next_color, depth);
        sierpinki(&context, [left_mid, left, bottom_mid], next_color, depth);
        sierpinki(&context, [right_mid, bottom_mid, right], next_color, depth);
    }
}

fn draw_triangle(
    context: &web_sys::CanvasRenderingContext2d,
    points: [(f64, f64); 3],
    color: (u8, u8, u8),
) {
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

fn midpoint(point_1: (f64, f64), point_2: (f64, f64)) -> (f64, f64) {
    (
        ((point_1.0 + point_2.0) / 2.0),
        ((point_1.1 + point_2.1) / 2.0),
    )
}

fn get_color(depth: u8) -> (u8, u8, u8) {
    let mut rng = thread_rng();
    let range = rand::distributions::Uniform::new(0, 255);

    let color = (
        range.sample(&mut rng),
        range.sample(&mut rng),
        range.sample(&mut rng),
    );

    return color;
}
