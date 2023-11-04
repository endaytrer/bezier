use bezier::linalg::{Vec2, Vec4};
use bezier::canvas::BezierCanvas;
use bezier::types::colortype::RGB;


pub struct VertexIn {
    pub xy: Vec2,
    pub color: Vec4,
    pub uv: Vec2,
}
pub struct ShaderUniform {
    pub texture: BezierCanvas<u32, RGB>
}