use crate::linalg::Linear;

use super::MeshOut;

pub trait VertexShader {
    type In;
    type Out: Linear<f32>;
    fn shade(vert: &Self::In) -> MeshOut<Self::Out>;
}