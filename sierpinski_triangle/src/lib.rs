use rand::prelude::*;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}
impl From<(f64, f64)> for Point {
    fn from(value: (f64, f64)) -> Self {
        Point {
            x: value.0,
            y: value.1,
        }
    }
}

#[derive(Clone, Copy)]
struct Triangle {
    top: Point,
    left: Point,
    right: Point,
}
impl Triangle {
    pub fn new(top: Point, left: Point, right: Point) -> Self {
        Triangle { top, left, right }
    }
}

#[derive(Clone, Copy)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl From<(u8, u8, u8)> for Rgb {
    fn from(value: (u8, u8, u8)) -> Self {
        Rgb {
            r: value.0,
            g: value.1,
            b: value.2,
        }
    }
}

impl Into<String> for Rgb {
    fn into(self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
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

    let first_triangle = Triangle::new(
        (300.0, 0.0).into(),
        (0.0, 600.0).into(),
        (600.0, 600.0).into(),
    );

    sierpinski(&context, first_triangle, (0, 255, 0).into(), 5);

    Ok(())
}
fn sierpinski(
    context: &web_sys::CanvasRenderingContext2d,
    triangle: Triangle,
    color: Rgb,
    depth: u8,
) {
    draw_triangle(&context, triangle, color);
    let depth = depth - 1;
    if depth > 0 {
        let mut rng = thread_rng();
        let next_color: Rgb = (
            rng.gen_range(0..255),
            rng.gen_range(0..255),
            rng.gen_range(0..255),
        )
            .into();
        let left_middle = midpoint(triangle.top, triangle.left);
        let right_middle = midpoint(triangle.top, triangle.right);
        let bottom_middle = midpoint(triangle.left, triangle.right);
        sierpinski(
            &context,
            Triangle::new(triangle.top.into(), left_middle.into(), right_middle.into()),
            next_color,
            depth,
        );
        sierpinski(
            &context,
            Triangle::new(
                left_middle.into(),
                triangle.left.into(),
                bottom_middle.into(),
            ),
            next_color,
            depth,
        );
        sierpinski(
            &context,
            Triangle::new(
                right_middle.into(),
                bottom_middle.into(),
                triangle.right.into(),
            ),
            next_color,
            depth,
        );
    }
}

fn midpoint(from: Point, to: Point) -> (f64, f64) {
    ((from.x + to.x) / 2.0, (from.y + to.y) / 2.0)
}

fn draw_triangle(context: &web_sys::CanvasRenderingContext2d, triangle: Triangle, color: Rgb) {
    let color_str: String = color.into();
    context.move_to(triangle.top.x, triangle.top.y);
    context.begin_path();
    context.line_to(triangle.left.x, triangle.left.y);
    context.line_to(triangle.right.x, triangle.right.y);
    context.line_to(triangle.top.x, triangle.top.y);
    context.close_path();
    context.stroke();
    context.fill();
    context.set_fill_style(&wasm_bindgen::JsValue::from_str(&color_str));
}
