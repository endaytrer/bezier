use bezier::types::blend::BlendMode;
use bezier::canvas::BezierCanvas;
use bezier::linalg::*;
use bezier::types::colortype::*;
use bezier::convert::PNGCompatible;
use frag::FragShader;
use vert::VertShader;
use types::{VertexIn, ShaderUniform};

mod types;
mod vert;
mod frag;
fn main() {

    let mut canvas = BezierCanvas::<u32, RGBA>::new(1200, 800);
    let texture = BezierCanvas::<u32, RGB>::from_png("avatar.png");
    let mut vertices: Vec<VertexIn> = Vec::new();
    for _ in 0..1000 {

        let pos0 = Vec2::new(rand::random(), rand::random());
        let pos1 = Vec2::new(rand::random(), rand::random());
        let pos2 = Vec2::new(rand::random(), rand::random());
        vertices.push(VertexIn {
            xy: pos0,
            uv: Vec2::new(0.5, 0.25),
            color: Vec4::new(1.0, 0.0, 1.0, 1.0)
        });
        
        vertices.push(VertexIn {
            xy: pos1,
            uv: Vec2::new(0.25, 0.75),
            color: Vec4::new(1.0, 1.0, 0.0, 1.0)
        });
        vertices.push(VertexIn {
            xy: pos2,
            uv: Vec2::new(0.75, 0.75),
            color: Vec4::new(0.0, 1.0, 1.0, 1.0)
        });
    }
    canvas.shade::<VertexIn, ShaderUniform, BVec<f32, 6>, VertShader, FragShader>(&vertices,
        &&ShaderUniform {
            texture,
        },
        BlendMode::Alpha
    );
    let poses = [
        Vec2{v: [0.25, 0.25]},
        Vec2{v: [0.325, 0.325]},
        Vec2{v: [0.75, 0.25]},
        Vec2{v: [0.875, 0.375]},
        Vec2{v: [1.0, 0.5]},
        Vec2{v: [-0.0625, 0.375]},
        Vec2{v: [0.375, 0.95]}
    ];
    for i in (1..poses.len()).step_by(3) {
        canvas.stroke_bezier::<4>(&poses[i - 1..i + 3], &RGBA {r: 255, g: 255, b: 255, a: 255}, 50, BlendMode::Alpha);
    }
    canvas.export_png("triangle.png");
}