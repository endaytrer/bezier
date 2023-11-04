use std::marker::PhantomData;

use crate::types::colortype::{InternalColorType, ColorType};
use crate::linalg::{Linear, Vec2};



pub struct VertexOut<T: Linear<f32>> {
    pub coord: Vec2,
    pub varying: T
}
impl <T: Linear<f32>> VertexOut<T> {
    pub fn new(coord: Vec2, varying: T) -> Self {
        VertexOut {
            coord,
            varying
        }
    }
}
pub struct FragOut<InternalType: InternalColorType, ExternalType: ColorType<InternalType>> {
    pub color: ExternalType,
    pub depth: f32,
    phantom_data: PhantomData<InternalType>
}

impl <InternalType: InternalColorType, ExternalType: ColorType<InternalType>> FragOut<InternalType, ExternalType> {
    pub fn new(color: ExternalType, depth: f32) -> Self {
        FragOut { color: color, depth: depth, phantom_data: PhantomData }
    }
}

pub trait VertexShader {
    type Attribute: Sync;
    type Uniform: Sync;
    type Out: Linear<f32> + Send + Sync;
    fn shade(vert: &Self::Attribute, uniform: &Self::Uniform) -> VertexOut<Self::Out>;
}
pub trait FragmentShader {
    type Uniform: Sync;
    type In: Linear<f32> + Send + Sync;
    type InternalType: InternalColorType;
    type ExternalType: ColorType<Self::InternalType>;
    fn shade(attribute: &Self::In, uniform: &Self::Uniform) -> FragOut<Self::InternalType, Self::ExternalType>;
}
