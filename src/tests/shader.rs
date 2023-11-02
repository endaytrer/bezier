use crate::{shader::{vert_shader::VertexShader, VertexOut, frag_shader::FragmentShader, FragOut}, linalg::{Vec2, BVec, Vec4}, types::{RGBA, blend::BlendMode, ColorType, RGB}, BezierCanvasFactory, BezierCanvas, texture::{LinearFilter, ClampToEdge}, convert::PNGCompatible};

pub struct VS {}

pub struct SU {
    texture: BezierCanvas<u32, RGB>
}

pub struct VIn {
    xy: Vec2,
    color: Vec4,
    uv: Vec2,
}
impl VertexShader for VS {
    type Attribute = VIn;

    type Uniform = SU;
    type Out = BVec<f32, 6>;

    fn shade(attr: &Self::Attribute, _uniform: &Self::Uniform) -> VertexOut<Self::Out> {
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

pub struct FS {}

impl FragmentShader for FS {
    type In = BVec<f32, 6>;

    type Uniform = SU;

    type InternalType = u32;

    type ExternalType = RGBA;

    fn shade(attribute: &Self::In, uniform: &Self::Uniform) -> FragOut<Self::InternalType, Self::ExternalType> {
        let color = uniform.texture.sample::<LinearFilter, ClampToEdge, ClampToEdge>(&attribute.xy());
        let color_2 = Vec4::new(attribute.v[2], attribute.v[3], attribute.v[4], attribute.v[5]);
        FragOut::new(RGBA::from_vec4(color.star(&color_2)), 0.0f32)
    }

}

#[test]
fn shade() {
    
    let mut canvas = BezierCanvasFactory::new()
        .set_size(1200, 800)
        .create_canvas::<u32, RGBA>();
    let texture = BezierCanvas::<u32, RGB>::from_png("avatar.png");
    let mut vertices: Vec<VIn> = Vec::new();
    for _ in 0..1000 {

        let pos0 = Vec2::new(rand::random(), rand::random());
        let pos1 = Vec2::new(rand::random(), rand::random());
        let pos2 = Vec2::new(rand::random(), rand::random());
        vertices.push(VIn {
            xy: pos0,
            uv: Vec2::new(0.5, 0.25),
            color: Vec4::new(1.0, 0.0, 1.0, 1.0)
        });
        
        vertices.push(VIn {
            xy: pos1,
            uv: Vec2::new(0.25, 0.75),
            color: Vec4::new(1.0, 1.0, 0.0, 1.0)
        });
        vertices.push(VIn {
            xy: pos2,
            uv: Vec2::new(0.75, 0.75),
            color: Vec4::new(0.0, 1.0, 1.0, 1.0)
        });
    }
    canvas.shade::<VIn, SU, BVec<f32, 6>, VS, FS>(&vertices,
        &SU {
            texture,
        },
        BlendMode::Alpha
    );
    let poses = [
        Vec2{v: [0.25, 0.25]},
        Vec2{v: [0.325, 0.325]},
        Vec2{v: [0.75, 0.25]},
        Vec2{v: [0.875, 0.375]},
        Vec2{v: [1.0, 0.5]},
        Vec2{v: [-0.0625, 0.375]},
        Vec2{v: [0.375, 0.95]}
    ];
    for i in (1..poses.len()).step_by(3) {
        canvas.stroke_bezier::<4>(&poses[i - 1..i + 3], &RGBA {r: 255, g: 255, b: 255, a: 255}, 50, BlendMode::Alpha);
    }
    canvas.export_png("target/debug/examples/triangle.png");
}