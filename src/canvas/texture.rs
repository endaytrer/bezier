use crate::types::colortype::{InternalColorType, ColorType};
use crate::texture::{SampleFilter, NearestFilter, LinearFilter, CubicFilter, Wrapping, WrapRepeat, WrapClampToEdge};
use crate::linalg::{Vec2, Vec4};
use crate::canvas::BezierCanvas;
/*
    a little different when treated as texture and canvas,

    when treated as a texture (indexed by uv), (0.0, 0.0) is at the top-left corner color point, and (1.0, 1.0) is at the bottom-right corner color points.
        the texture is considered as a 2D grid of color points, each point is a pixel; the color is sampled from an interpolation of nearby color points

    when treated as a canvas, (0.0, 0.0) is at the top-left corner of the top-left corner pixel, and (1.0, 1.0) is at the bottom-right bottom-right corner pixel.
        the canvas is considered as a continuous field with square pixels, whose edge is on integral xy points. if a point we need to fill is within a pixel, we fill this pixel.

    Hereby filtering is treated as a texture.
*/
// fn point_to_uv(pnt: usize, max: usize) -> f32 {
//     (pnt as f32) / ((max - 1) as f32)
// }

fn uv_to_point(uv: f32, max: usize) -> f32 {
    uv * ((max - 1) as f32)
}

impl <InternalType: InternalColorType, ExternalType: ColorType<InternalType>> SampleFilter<InternalType, ExternalType> for NearestFilter {
    fn sample(uv: Vec2, texture: &BezierCanvas<InternalType, ExternalType>) -> Vec4 {
        texture.get_pixel(
            uv_to_point(uv.x(), texture.width).round() as usize,
            uv_to_point(uv.y(), texture.height).round() as usize
        ).to_vec4()
    }
}
impl <InternalType: InternalColorType, ExternalType: ColorType<InternalType>> SampleFilter<InternalType, ExternalType> for LinearFilter {
    fn sample(uv: Vec2, texture: &BezierCanvas<InternalType, ExternalType>) -> Vec4 {
        let x = uv_to_point(uv.x(), texture.width);
        let x0 = x.floor() as usize;
        let x1 = x.ceil() as usize;

        let t = x - (x0 as f32);
        let t1 = t;
        let t0 = 1.0 - t1;

        let y = uv_to_point(uv.y(), texture.height);
        let y0 = y.floor() as usize;
        let y1 = y.ceil() as usize;

        let u = y - (y0 as f32);
        let u1 = u;
        let u0 = 1.0 - u1;

        let p00 = texture.get_pixel(x0, y0).to_vec4();
        let p01 = texture.get_pixel(x0, y1).to_vec4();
        let p10 = texture.get_pixel(x1, y0).to_vec4();
        let p11 = texture.get_pixel(x1, y1).to_vec4();
        t0 * u0 * p00 + t1 * u0 * p10 +  t0 * u1 * p01 + t1 * u1 * p11
    }
}
impl <InternalType: InternalColorType, ExternalType: ColorType<InternalType>> SampleFilter<InternalType, ExternalType> for CubicFilter {
    fn sample(uv: Vec2, texture: &BezierCanvas<InternalType, ExternalType>) -> Vec4 {
        let x = uv_to_point(uv.x(), texture.width);
        let x0 = x.floor() as usize;
        let x1 = x.ceil() as usize;

        let t = x - (x0 as f32);
        let t1 = -2.0 * t * t * t + 3.0 * t * t;
        let t0 = 1.0 - t1;

        let y = uv_to_point(uv.y(), texture.height);
        let y0 = y.floor() as usize;
        let y1 = y.ceil() as usize;

        let u = y - (y0 as f32);
        let u1 = -2.0 * u * u * u + 3.0 * u * u;
        let u0 = 1.0 - u1;

        let p00 = texture.get_pixel(x0, y0).to_vec4();
        let p01 = texture.get_pixel(x0, y1).to_vec4();
        let p10 = texture.get_pixel(x1, y0).to_vec4();
        let p11 = texture.get_pixel(x1, y1).to_vec4();
        t0 * u0 * p00 + t1 * u0 * p10 +  t0 * u1 * p01 + t1 * u1 * p11
    }
}


impl Wrapping for WrapRepeat {
    fn wrap(x: f32) -> f32 {
        x - x.floor()
    }
}
impl Wrapping for WrapClampToEdge {
    fn wrap(x: f32) -> f32 {
        x.clamp(0.0f32, 1.0f32)
    }
}

impl <InternalType: InternalColorType, ExternalType: ColorType<InternalType>> BezierCanvas<InternalType, ExternalType> {
    pub fn sample<TextureFilter: SampleFilter<InternalType, ExternalType>, WrapS: Wrapping, WrapT: Wrapping>(&self, uv: &Vec2) -> Vec4 {
        TextureFilter::sample(Vec2::new(WrapS::wrap(uv.x()), WrapT::wrap(uv.y())), self)
    }
}