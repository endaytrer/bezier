use bezier::shading::{FragmentShader, FragOut};
use bezier::texture::{LinearFilter, WrapClampToEdge};
use bezier::types::colortype::*;
use bezier::linalg::*;
use crate::types::ShaderUniform;


pub struct FragShader {}

impl FragmentShader for FragShader {
    type In = BVec<f32, 6>;

    type Uniform = ShaderUniform;

    type InternalType = u32;

    type ExternalType = RGBA;

    fn shade(attribute: &Self::In, uniform: &Self::Uniform) -> FragOut<Self::InternalType, Self::ExternalType> {
        let color = uniform.texture.sample::<LinearFilter, WrapClampToEdge, WrapClampToEdge>(&attribute.xy());
        let color_2 = Vec4::new(attribute.v[2], attribute.v[3], attribute.v[4], attribute.v[5]);
        FragOut::new(RGBA::from_vec4(color.star(&color_2)), 0.0f32)
    }
}
