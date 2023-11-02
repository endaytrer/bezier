
use crate::{types::{InternalColorType, ColorType}, linalg::Linear};

use super::FragOut;

pub trait FragmentShader {
    type AttrType: Linear<f32>;
    type InternalType: InternalColorType;
    type ExternalType: ColorType<Self::InternalType>;
    fn shade(attribute: &Self::AttrType) -> FragOut<Self::InternalType, Self::ExternalType>;
}
