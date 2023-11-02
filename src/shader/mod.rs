use std::marker::PhantomData;

use crate::{linalg::{Linear, Vec2, Matrix2, Det}, types::{InternalColorType, ColorType, blend::BlendMode}, BezierCanvas};

use self::{vert_shader::VertexShader, frag_shader::FragmentShader};

pub mod vert_shader;
pub mod frag_shader;

pub struct MeshOut<T: Linear<f32>> {
    pub coord: Vec2,
    pub attribute: T
}
impl <T: Linear<f32>> MeshOut<T> {
    pub fn new(coord: Vec2, attribute: T) -> Self {
        MeshOut {
            coord,
            attribute
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
impl <InternalType: InternalColorType, ExternalType: ColorType<InternalType>> BezierCanvas<InternalType, ExternalType> {
    pub fn shade<In, Intermediate: Linear<f32>, VertShader: VertexShader<In = In, Out = Intermediate>, FragShader: FragmentShader<AttrType = Intermediate, InternalType = InternalType, ExternalType = ExternalType>>(&mut self, vertices: &[In], blend_mode: BlendMode) {
        let mut depth_buffer = vec![vec![f32::NEG_INFINITY; self.width]; self.height];
        let mut out: Vec<MeshOut<Intermediate>> = Vec::new();
        for v in vertices {
            out.push(VertShader::shade(v));
        }
        for i in (0..out.len()).step_by(3) {
            let v0 = out[3 * i + 0].coord;
            let v1 = out[3 * i + 1].coord;
            let v2 = out[3 * i + 2].coord;
            let attr0 = out[3 * i + 0].attribute;
            let attr1 = out[3 * i + 1].attribute;
            let attr2 = out[3 * i + 2].attribute;
            let min_x = (v0.x()
                .clamp(0f32, v1.x())
                .clamp(0f32, v2.x()) * (self.width as f32))
                .floor() as usize;
            let max_x = (v0.x()
                .clamp(v1.x(), 1f32)
                .clamp(v2.x(), 1f32) * (self.width as f32))
                .ceil() as usize;
            let min_y = (v0.y()
                .clamp(0f32, v1.y())
                .clamp(0f32, v2.y()) * (self.height as f32))
                .floor() as usize;
            let max_y = (v0.y()
                .clamp(v1.y(), 1f32)
                .clamp(v2.y(), 1f32) * (self.height as f32))
                .ceil() as usize;

            let mat = Matrix2 {v: [(v1 - v0).v, (v2 - v0).v]}.transpose();
            let det_mat = mat.det();
            if det_mat.abs() < f32::EPSILON {
                return;
            }
            for y in min_y..max_y {
                for x in min_x..max_x {
                    let coord = Vec2 {v: [(x as f32) / (self.width as f32), (y as f32) / (self.height as f32)]};
                    let ans = coord - v0;
                    let det_t = Matrix2 {
                        v: [[ans.x(), mat.v[0][1]],
                            [ans.y(), mat.v[1][1]]]
                    }.det();
                    let det_u = Matrix2 {
                        v: [[mat.v[0][0], ans.x()],
                            [mat.v[1][0], ans.y()]]
                    }.det();

                    let t = det_t / det_mat;
                    let u = det_u / det_mat;
                    if t < 0f32 || u < 0f32 || (1f32 - t - u) < 0f32 {
                        continue;
                    }
                    let attrib = 
                        attr0 * (1f32 - t - u) +
                        attr1 * t +
                        attr2 * u;
                    let shaded = FragShader::shade(&attrib);
                    if shaded.depth > depth_buffer[y][x] {
                        depth_buffer[y][x] = shaded.depth;
                        self.set_pixel(x, y, &shaded.color, blend_mode);
                    }
                }
            }
        }
    }
}