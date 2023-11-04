use bezier::shading::{VertexShader, VertexOut};
use bezier::linalg::*;

use crate::types::{VertexIn, ShaderUniform};

pub struct VertShader {}
impl VertexShader for VertShader {
    type Attribute = VertexIn;

    type Uniform = ShaderUniform;

    type Out = BVec<f32, 6>;

    fn shade(attr: &Self::Attribute, _uniform: &Self::Uniform) -> bezier::shading::VertexOut<Self::Out> {
        let ans = VertexOut::<BVec<f32, 6>> {
            coord: attr.xy,
            varying: BVec {
                v: [
                    attr.uv.x(),
                    attr.uv.y(),
                    attr.color.x(),
                    attr.color.y(),
                    attr.color.z(),
                    attr.color.w(),
                ]
            },
        };
        ans
    }
}