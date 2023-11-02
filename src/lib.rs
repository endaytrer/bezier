
use std::marker::PhantomData;
use linalg::Vec2;
use num::traits::Zero;
mod types;
mod convert;
pub mod shader;
pub mod linalg;
use types::{ColorType, InternalColorType, blend::BlendMode};

pub struct BezierCanvasFactory {
    width: Option<usize>,
    height: Option<usize>,
}

impl BezierCanvasFactory {
    pub fn new() -> BezierCanvasFactory {
        BezierCanvasFactory { width: None, height: None}
    }

    pub fn set_size(&mut self, width: usize, height: usize) -> &mut BezierCanvasFactory {
        self.width = Some(width);
        self.height = Some(height);
        self
    }
    
    pub fn create_canvas<InternalType: InternalColorType, ExternalType: ColorType<InternalType>>(&self) -> BezierCanvas<InternalType, ExternalType> {
        BezierCanvas {
            width: self.width.unwrap(),
            height: self.height.unwrap(),
            pixels: vec![Zero::zero(); self.width.unwrap() * self.height.unwrap()],
            external_type: PhantomData
        }
    }
}

pub struct BezierCanvas<InternalType: InternalColorType, ExternalType: ColorType<InternalType>> {
    width: usize,
    height: usize,
    pixels: Vec<InternalType>,
    external_type: PhantomData<ExternalType>
}
const MAX_PASCAL: usize = 10;
const C: [[usize; MAX_PASCAL]; MAX_PASCAL] = [
    [1, 0,  0,  0,   0,   0, 0,  0,  0, 0],
    [1, 1,  0,  0,   0,   0, 0,  0,  0, 0],
    [1, 2,  1,  0,   0,   0, 0,  0,  0, 0],
    [1, 3,  3,  1,   0,   0, 0,  0,  0, 0],
    [1, 4,  6,  4,   1,   0, 0,  0,  0, 0],
    [1, 5, 10, 10,   5,   1, 0,  0,  0, 0],
    [1, 6, 15, 20,  15,   6, 1,  0,  0, 0],
    [1, 7, 21, 35,  35,  21, 7,  1,  0, 0],
    [1, 8, 28, 56,  70,  56, 28, 8,  1, 0],
    [1, 9, 36, 84, 126, 126, 84, 36, 9, 1]
];

impl <InternalType: InternalColorType, ExternalType: ColorType<InternalType>> BezierCanvas<InternalType, ExternalType> {
    pub fn get_pixel(&self, x: usize, y: usize) -> ExternalType {
        assert!(x < self.width && y < self.height);
        ExternalType::from_value(self.pixels[y * self.width + x])
    }

    fn set_pixel(&mut self, x: usize, y: usize, pixel: &ExternalType, blend_mode: BlendMode) {
        self.pixels[y * self.width + x] = pixel.blend(self.pixels[y * self.width + x], blend_mode);
    }

    pub fn fill_rect(&mut self, pos: &Vec2, size: &Vec2, color: &ExternalType, blend_mode: BlendMode) {
        let x_0: usize = (pos.x()
            .clamp(0.0f32, 1.0f32) * (self.width as f32))
            .floor() as usize;
        let x_1: usize = ((pos.x() + size.x())
            .clamp(0.0f32, 1.0f32) * (self.width as f32))
            .ceil() as usize;
        let y_0: usize = (pos.y()
            .clamp(0.0f32, 1.0f32) * (self.height as f32))
            .floor() as usize;
        let y_1: usize = ((pos.y() + size.y())
            .clamp(0.0f32, 1.0f32) * (self.height as f32))
            .ceil() as usize;
        for y in y_0..y_1 {
            for x in x_0..x_1 {
                self.set_pixel(x, y, color, blend_mode);
            }   
        } 
    }
    pub fn fill_oval(&mut self, pos: &Vec2, size: &Vec2, color: &ExternalType, blend_mode: BlendMode) {
        let x_0 = ((pos.x() - size.x())
            .clamp(0.0f32, 1.0f32) * (self.width as f32))
            .floor() as usize;
        let x_1 = ((pos.x() + size.x())
            .clamp(0.0f32, 1.0f32) * (self.width as f32))
            .ceil() as usize;
        let y_0 = ((pos.y() - size.y())
            .clamp(0.0f32, 1.0f32) * (self.height as f32))
            .floor() as usize;
        let y_1 = ((pos.y() + size.y())
            .clamp(0.0f32, 1.0f32) * (self.height as f32))
            .ceil() as usize;
        
        let w2 = size.x() * size.x();
        let h2 = size.y() * size.y();
        for y in y_0..y_1 {
            let rel_y: f32 = ((y as f32) / (self.height as f32)) - pos.y();
            let y2 = rel_y * rel_y;
            for x in x_0..x_1 {
                let rel_x: f32 = ((x as f32) / (self.width as f32)) - pos.x();
                let x2 = rel_x * rel_x;
                if x2 / w2 + y2 / h2 < 1f32 {
                    self.set_pixel(x, y, color, blend_mode)
                }
            }
        }
    }

    pub fn fill_circle(&mut self, pos: &Vec2, radius: f32, color: &ExternalType, blend_mode: BlendMode) {
        self.fill_oval(pos, &Vec2 {v: [radius, radius]}, color, blend_mode);
    }

    fn stroke_line_gentle(&mut self, pos0: &Vec2, pos1: &Vec2, color: &ExternalType, blend_mode: BlendMode) {
        // with abs(slope) < 1
        let x_0 = (pos0.x()
            .clamp(0.0f32, pos1.x()) * (self.width as f32))
            .floor() as usize;
        let x_1 = (pos0.x()
            .clamp(pos1.x(), 1.0f32) * (self.width as f32))
            .ceil() as usize;
        for x in x_0..x_1 {
            let y = (pos1.y() - pos0.y()) / (pos1.x() - pos0.x()) * ((x as f32) / (self.width as f32)) +
             (pos0.y() - (pos1.y() - pos0.y()) / (pos1.x() - pos0.x()) * pos0.x());
            if y >= 0.0f32 && y < 1.0f32 {
                self.set_pixel(x, (y * (self.height as f32)) as usize, color, blend_mode);
            }
        }
    }
    fn stroke_line_steep(&mut self, pos0: &Vec2, pos1: &Vec2, color: &ExternalType, blend_mode: BlendMode) {
        // with abs(slope) < 1
        let y_0 = (pos0.y()
            .clamp(0.0f32, pos1.y()) * (self.height as f32))
            .floor() as usize;
        let y_1 = (pos0.y()
            .clamp(pos1.y(), 1.0f32) * (self.height as f32))
            .ceil() as usize;
        for y in y_0..y_1 {
            let x = (pos1.x() - pos0.x()) / (pos1.y() - pos0.y()) * ((y as f32) / (self.height as f32)) +
             (pos0.x() - (pos1.x() - pos0.x()) / (pos1.y() - pos0.y()) * pos0.y());
            if x >= 0.0f32 && x < 1.0f32 {
                self.set_pixel((x * (self.width as f32)) as usize , y, color, blend_mode);
            }
        }
    }
    pub fn stroke_line(&mut self, pos0: &Vec2, pos1: &Vec2, color: &ExternalType, blend_mode: BlendMode) {
        let dx = (pos1.x() - pos0.x()).abs();
        let dy = (pos1.y() - pos0.y()).abs();
        if dx > dy {
            self.stroke_line_gentle(pos0, pos1, color, blend_mode);
        }
        else {
            self.stroke_line_steep(pos0, pos1, color, blend_mode);
        }
    }
    pub fn stroke_bezier<const N: usize>(&mut self, poses: &[Vec2], color: &ExternalType, stops: usize, blend_mode: BlendMode) {
        // draw (N - 1)-th order bezier curve
        assert!(N < MAX_PASCAL);
        let mut prev_ans: Option<Vec2> = None;
        for i in 0..stops {
            let t = (i as f32) / ((stops - 1) as f32);
            let neg_t = 1f32 - t;

            let mut coefficients_t = [1f32; N];
            let mut coefficients_neg_t = [1f32; N];
            for j in 1..N {
                coefficients_t[j] = coefficients_t[j - 1] * t;
                coefficients_neg_t[N - j - 1] = coefficients_neg_t[N - j] * neg_t
            }
            let mut ans = Vec2::zero();
            for j in 0..N {
                // N - 1 choose j
                ans = ans + C[N - 1][j] as f32 * coefficients_t[j] * coefficients_neg_t[j] * poses[j];
            }
            if let Some(prev) = prev_ans {
                self.stroke_line(&prev, &ans, color, blend_mode);
            }
            prev_ans = Some(ans);
        }
    }
}


#[cfg(test)]
mod tests;