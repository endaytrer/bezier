use crate::types::colortype::{InternalColorType, ColorType};
use crate::linalg::{Vec2, Vec4};
use crate::canvas::BezierCanvas;

pub struct NearestFilter {}
pub struct LinearFilter {}
pub struct CubicFilter {}

pub trait SampleFilter<InternalType: InternalColorType, ExternalType: ColorType<InternalType>> {
    fn sample(uv: Vec2, texture: &BezierCanvas<InternalType, ExternalType>) -> Vec4;
}

pub struct WrapRepeat {}
pub struct WrapClampToEdge {}

pub trait Wrapping {
    fn wrap(x: f32) -> f32;
}