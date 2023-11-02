use num::One;

use crate::BezierCanvasFactory;
use crate::linalg::{Vec2, Matrix2, Det};
use crate::types::{RGB, R, RGBA, blend::BlendMode};

mod shader;
#[test]
fn init_canvas() {
    let canvas = BezierCanvasFactory::new()
        .set_size(1200, 800)
        .create_canvas::<u32, RGB>();

    assert_eq!(canvas.get_pixel(0, 0), RGB {r: 0, g: 0, b: 0});
    assert_eq!(canvas.get_pixel(1199, 799), RGB {r: 0, g: 0, b: 0});
    canvas.export_png("target/debug/examples/hello.png");
}
#[test]
fn rectangle_and_lines() {
    let mut canvas = BezierCanvasFactory::new()
        .set_size(1200, 800)
        .create_canvas::<u32, RGB>();

    canvas.fill_rect(&Vec2 {v: [0.1667f32, 0.125f32]}, &Vec2{v: [0.25f32, 0.25f32]}, &RGB { r: 255, g: 0, b: 0 }, BlendMode::Override);
    canvas.export_png("target/debug/examples/rect.png");
    assert_eq!(canvas.get_pixel(0, 0), RGB {r: 0, g: 0, b: 0});
    assert_eq!(canvas.get_pixel(200, 200), RGB {r: 255, g: 0, b: 0});
}
#[test]
#[should_panic]
fn read_outside() {
    let canvas = BezierCanvasFactory::new()
        .set_size(1200, 800)
        .create_canvas::<u8, R>();
    canvas.get_pixel(1200, 0);
}

#[test]
fn linear_algebra() {
    let identity = Matrix2::one();
    let x = Vec2 {v: [0.75, 0.25]};
    assert_eq!(x + x, Vec2 {v: [1.5, 0.5]});
    assert_eq!(identity * x, x);
    assert_eq!(x * identity, x);
    assert_eq!(x * identity * x, x * x);
    assert_eq!(identity.det(), 1.0f32);
    assert_eq!((identity * 5.0f32).det(), 25.0f32);
}

#[test]
fn bezier_curve() {
    let mut canvas = BezierCanvasFactory::new()
        .set_size(400, 400)
        .create_canvas::<u32, RGBA>();

    canvas.fill_oval(&Vec2{v: [0.5, 0.5]}, &Vec2{v: [0.25, 0.375]}, &RGBA { r: 255, g: 0, b: 0, a: 200 }, BlendMode::Alpha);
    let poses = [
        Vec2{v: [0.25, 0.25]},
        Vec2{v: [0.325, 0.325]},
        Vec2{v: [0.75, 0.25]},
        Vec2{v: [0.875, 0.375]},
        Vec2{v: [1.0, 0.5]},
        Vec2{v: [-0.0625, 0.375]},
        Vec2{v: [0.375, 0.95]}
    ];
    for pnt in poses {
        canvas.fill_circle(&pnt, 0.01, &RGBA { r: 255, g: 100, b: 100, a: 235 }, BlendMode::Alpha);
    }
    for i in (1..poses.len()).step_by(3) {
        canvas.stroke_line(&poses[i - 1], &poses[i], &RGBA{r: 100, g: 100, b: 100, a: 255}, BlendMode::Alpha);
        canvas.stroke_line(&poses[i + 1], &poses[i + 2], &RGBA{r: 100, g: 100, b: 100, a: 255}, BlendMode::Alpha);
    }
    for i in (1..poses.len()).step_by(3) {
        canvas.stroke_bezier::<4>(&poses[i - 1..i + 3], &RGBA {r: 255, g: 255, b: 255, a: 255}, 50, BlendMode::Alpha);
    }
    canvas.export_png("target/debug/examples/bezier.png");

}

#[test]
fn blend_modes() {
    let mut canvas = BezierCanvasFactory::new()
        .set_size(400, 400)
        .create_canvas::<u32, RGB>();

    canvas.fill_rect(&Vec2 { v: [0.0, 0.0]}, &Vec2 { v: [1.0, 1.0]}, &RGB { r: 0, g: 255, b: 0 }, BlendMode::Override);
    canvas.fill_rect(&Vec2 { v: [0.25, 0.25]}, &Vec2 { v: [0.375, 0.375]}, &RGB { r: 255, g: 255, b: 0 }, BlendMode::Screen);
    canvas.fill_rect(&Vec2 { v: [0.375, 0.375]}, &Vec2 { v: [0.375, 0.375]}, &RGB { r: 255, g: 0, b: 255 }, BlendMode::Multiply);

    assert_eq!(canvas.get_pixel(50, 50), RGB{r: 0, g: 255, b: 0});
    assert_eq!(canvas.get_pixel(125, 125), RGB{r: 255, g: 255, b: 0});
    assert_eq!(canvas.get_pixel(200, 200), RGB{r: 255, g: 0, b: 0});
    assert_eq!(canvas.get_pixel(275, 275), RGB{r: 0, g: 0, b: 0});

    canvas.export_png("target/debug/examples/blend.png");

}