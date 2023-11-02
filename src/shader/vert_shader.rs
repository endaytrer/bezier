use crate::linalg::Linear;

use super::VertexOut;

pub trait VertexShader {
    type Attribute;
    type Uniform;
    type Out: Linear<f32>;
    fn shade(vert: &Self::Attribute, uniform: &Self::Uniform) -> VertexOut<Self::Out>;
}