
use crate::linalg::{Linear, Vec2, Matrix2, Det};
use crate::types::{
    colortype::{InternalColorType, ColorType},
    blend::BlendMode
};
use crate::canvas::BezierCanvas;
use crate::shading::{VertexShader, FragmentShader, VertexOut};

use rayon::prelude::*;
impl <InternalType: InternalColorType, ExternalType: ColorType<InternalType>> BezierCanvas<InternalType, ExternalType> {
    pub fn shade<
        Attribute: Sync,
        Uniform: Sync,
        Intermediate: Linear<f32> + Send + Sync,
        VertShader: VertexShader<Attribute = Attribute, Out = Intermediate, Uniform = Uniform>,
        FragShader: FragmentShader<In = Intermediate, Uniform = Uniform, InternalType = InternalType, ExternalType = ExternalType>>
        (&mut self, attribute: &[Attribute], uniform: &Uniform, blend_mode: BlendMode) {

        let mut depth_buffer = vec![f32::NEG_INFINITY; self.width * self.height];
        let out: Vec<VertexOut<Intermediate>> = attribute.into_par_iter()
            .map(|v| VertShader::shade(v, uniform))
            .collect();
        for i in (0..out.len()).step_by(3) {
            let v0 = out[i + 0].coord;
            let v1 = out[i + 1].coord;
            let v2 = out[i + 2].coord;
            let attr0 = out[i + 0].varying;
            let attr1 = out[i + 1].varying;
            let attr2 = out[i + 2].varying;
            let min_x = Self::xy_to_pixel(v0.x()
                .clamp(0f32, v1.x())
                .clamp(0f32, v2.x()), self.width);
            let max_x = Self::xy_to_pixel(v0.x()
                .clamp(v1.x(), 1f32)
                .clamp(v2.x(), 1f32), self.width);
            let min_y = Self::xy_to_pixel(v0.y()
                .clamp(0f32, v1.y())
                .clamp(0f32, v2.y()), self.height);
            let max_y = Self::xy_to_pixel(v0.y()
                .clamp(v1.y(), 1f32)
                .clamp(v2.y(), 1f32), self.height);

            let mat = Matrix2 {v: [(v1 - v0).v, (v2 - v0).v]}.transpose();
            let det_mat = mat.det();
            if det_mat.abs() < f32::EPSILON {
                return;
            }
            let depth_chunks = depth_buffer.par_chunks_mut(self.width)
                .skip(min_y)
                .take(max_y + 1 - min_y);
            let pixel_chunks = self.pixels.par_chunks_mut(self.width)
                .skip(min_y)
                .take(max_y + 1 - min_y);

            depth_chunks.zip(pixel_chunks)
            .enumerate()
            .for_each(|(i, (depth_range, pixel_range))| {
                let y = i + min_y;
                let depth_iter = depth_range.par_iter_mut()
                    .skip(min_x)
                    .take(max_x + 1 - min_x);
                let pixel_iter = pixel_range.par_iter_mut()
                    .skip(min_x)
                    .take(max_x + 1 - min_x);

                depth_iter.zip(pixel_iter)
                    .enumerate()
                    .for_each(|(j, (depth, pixel))| {
                        let x = j + min_x;
                        let coord = Vec2::new(Self::pixel_to_xy(x, self.width), Self::pixel_to_xy(y, self.height));
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
                            return;
                        }
                        let attrib = 
                            attr0 * (1f32 - t - u) +
                            attr1 * t +
                            attr2 * u;
                        let shaded = FragShader::shade(&attrib, uniform);
                        if shaded.depth > *depth {
                            *depth = shaded.depth;
                            BezierCanvas::par_set_pixel(pixel, &shaded.color, blend_mode);
                        }
                    })
            });
        }
    }
}