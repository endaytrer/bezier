
use crate::{types::{InternalColorType, ColorType}, linalg::Linear};

use super::FragOut;

pub trait FragmentShader {
    type Uniform;
    type In: Linear<f32>;
    type InternalType: InternalColorType;
    type ExternalType: ColorType<Self::InternalType>;
    fn shade(attribute: &Self::In, uniform: &Self::Uniform) -> FragOut<Self::InternalType, Self::ExternalType>;
}
