use crate::{shader::{vert_shader::VertexShader, MeshOut, frag_shader::FragmentShader, FragOut}, linalg::{Vec2, Vec4}, types::{RGBA, blend::BlendMode}, BezierCanvasFactory};

pub struct VS {}
pub struct VIn {
    xy: Vec2,
    color: Vec4,
}
impl VertexShader for VS {
    type In = VIn;

    type Out = Vec4;

    fn shade(vert: &Self::In) -> MeshOut<Self::Out> {
        let ans = MeshOut{
            coord: vert.xy,
            attribute: vert.color,
        };
        ans
    }
}

pub struct FS {}

impl FragmentShader for FS {
    type AttrType = Vec4;

    type InternalType = u32;

    type ExternalType = RGBA;

    fn shade(attribute: &Self::AttrType) -> FragOut<Self::InternalType, Self::ExternalType> {
        FragOut::new(RGBA {r: attribute.x() as u8, g: attribute.y() as u8, b: attribute.z() as u8, a: attribute.w() as u8}, 0.0f32)
    }
}

#[test]
fn shade() {
    
    let mut canvas = BezierCanvasFactory::new()
        .set_size(1200, 800)
        .create_canvas::<u32, RGBA>();

    canvas.shade::<VIn, Vec4, VS, FS>(&[
        VIn {
            xy: Vec2{v: [0.5, 0.25]},
            color: Vec4{v: [255f32, 0f32, 0f32, 255f32]}
        },
        VIn {
            xy: Vec2{v: [0.25, 0.75]},
            color: Vec4{v: [0f32, 255f32, 0f32, 255f32]}
        },
        VIn {
            xy: Vec2{v: [0.75, 0.75]},
            color: Vec4{v: [0f32, 0f32, 255f32, 255f32]}
        }],
        BlendMode::Alpha
    );
    canvas.export_png("target/debug/examples/triangle.png");
}