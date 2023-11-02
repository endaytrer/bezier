use crate::{types::{InternalColorType, ColorType}, linalg::{Vec2, Vec4}, BezierCanvas};

pub trait SampleFilter<InternalType: InternalColorType, ExternalType: ColorType<InternalType>> {
    fn sample(uv: Vec2, texture: &BezierCanvas<InternalType, ExternalType>) -> Vec4;
}

pub struct NearestFilter {}
impl <InternalType: InternalColorType, ExternalType: ColorType<InternalType>> SampleFilter<InternalType, ExternalType> for NearestFilter {
    fn sample(uv: Vec2, texture: &BezierCanvas<InternalType, ExternalType>) -> Vec4 {
        texture.get_pixel(
            (uv.x() * ((texture.width - 1) as f32)).round() as usize,
            (uv.y() * ((texture.height - 1) as f32)).round() as usize
        ).to_vec4()
    }
}

pub struct LinearFilter {}
impl <InternalType: InternalColorType, ExternalType: ColorType<InternalType>> SampleFilter<InternalType, ExternalType> for LinearFilter {
    fn sample(uv: Vec2, texture: &BezierCanvas<InternalType, ExternalType>) -> Vec4 {
        let x0 = (uv.x() * ((texture.width - 1) as f32)).floor() as usize;
        let x1 = (uv.x() * ((texture.width - 1) as f32)).ceil() as usize;
        let t = uv.x() * ((texture.width - 1) as f32) - (x0 as f32);
        let t1 = t;
        let t0 = 1.0 - t1;
        let y0 = (uv.y() * ((texture.height - 1) as f32)).floor() as usize;
        let y1 = (uv.y() * ((texture.height - 1) as f32)).ceil() as usize;
        let u = uv.y() * ((texture.height - 1) as f32) - (y0 as f32);
        let u1 = u;
        let u0 = 1.0 - u1;
        let p00 = texture.get_pixel(x0, y0).to_vec4();
        let p01 = texture.get_pixel(x0, y1).to_vec4();
        let p10 = texture.get_pixel(x1, y0).to_vec4();
        let p11 = texture.get_pixel(x1, y1).to_vec4();
        t0 * u0 * p00 + t1 * u0 * p10 +  t0 * u1 * p01 + t1 * u1 * p11
    }
}
pub struct CubicFilter {}
impl <InternalType: InternalColorType, ExternalType: ColorType<InternalType>> SampleFilter<InternalType, ExternalType> for CubicFilter {
    fn sample(uv: Vec2, texture: &BezierCanvas<InternalType, ExternalType>) -> Vec4 {
        let x0 = (uv.x() * ((texture.width - 1) as f32)).floor() as usize;
        let x1 = (uv.x() * ((texture.width - 1) as f32)).ceil() as usize;
        let t = uv.x() * ((texture.width - 1) as f32) - (x0 as f32);
        let t1 = -2.0 * t * t * t + 3.0 * t * t;
        let t0 = 1.0 - t1;
        let y0 = (uv.y() * ((texture.height - 1) as f32)).floor() as usize;
        let y1 = (uv.y() * ((texture.height - 1) as f32)).ceil() as usize;
        let u = uv.y() * ((texture.height - 1) as f32) - (y0 as f32);
        let u1 = -2.0 * u * u * u + 3.0 * u * u;
        let u0 = 1.0 - u1;
        let p00 = texture.get_pixel(x0, y0).to_vec4();
        let p01 = texture.get_pixel(x0, y1).to_vec4();
        let p10 = texture.get_pixel(x1, y0).to_vec4();
        let p11 = texture.get_pixel(x1, y1).to_vec4();
        t0 * u0 * p00 + t1 * u0 * p10 +  t0 * u1 * p01 + t1 * u1 * p11
    }
}

pub trait Wrapping {
    fn wrap(x: f32) -> f32;
}

pub struct Repeat {}
impl Wrapping for Repeat {
    fn wrap(x: f32) -> f32 {
        x - x.floor()
    }
}
pub struct ClampToEdge {}
impl Wrapping for ClampToEdge {
    fn wrap(x: f32) -> f32 {
        x.clamp(0.0f32, 1.0f32)
    }
}


impl <InternalType: InternalColorType, ExternalType: ColorType<InternalType>> BezierCanvas<InternalType, ExternalType> {
    pub fn sample<TextureFilter: SampleFilter<InternalType, ExternalType>, WrapS: Wrapping, WrapT: Wrapping>(&self, uv: &Vec2) -> Vec4 {
        TextureFilter::sample(Vec2::new(WrapS::wrap(uv.x()), WrapT::wrap(uv.y())), self)
    }
}