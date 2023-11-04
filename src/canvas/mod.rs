mod convert;
mod shade;
mod texture;

use std::marker::PhantomData;
use num::traits::Zero;
use rayon::slice::ParallelSliceMut;
use rayon::prelude::*;

use crate::linalg::Vec2;
use crate::types::{
    colortype::{ColorType, InternalColorType},
    blend::BlendMode
};

pub struct BezierCanvas<InternalType: InternalColorType, ExternalType: ColorType<InternalType>> {
    pub width: usize,
    pub height: usize,
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
    pub fn new(width: usize, height: usize) -> Self {
        BezierCanvas {
            width: width,
            height: height,
            pixels: vec![Zero::zero(); width * height],
            external_type: PhantomData
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> ExternalType {
        ExternalType::from_value(self.pixels[y * self.width + x])
    }

    fn set_pixel(&mut self, x: usize, y: usize, pixel: &ExternalType, blend_mode: BlendMode) {
        self.pixels[y * self.width + x] = blend_mode.blend(self.pixels[y * self.width + x], pixel);
    }

    fn par_set_pixel(pixel: &mut InternalType, color: &ExternalType, blend_mode: BlendMode) {
        *pixel = blend_mode.blend(*pixel, color);
    }

    /*
        a little different when treated as texture and canvas,

        when treated as a texture (indexed by uv), (0.0, 0.0) is at the top-left corner color point, and (1.0, 1.0) is at the bottom-right corner color points.
            the texture is considered as a 2D grid of color points, each point is a pixel; the color is sampled from an interpolation of nearby color points

        when treated as a canvas, (0.0, 0.0) is at the top-left corner of the top-left corner pixel, and (1.0, 1.0) is at the bottom-right bottom-right corner pixel.
            the canvas is considered as a continuous field with square pixels, whose edge is on integral xy points. if a point we need to fill is within a pixel, we fill this pixel.
     */
    fn pixel_to_xy(pix: usize, max: usize) -> f32 {
        (pix as f32 + 0.5) / (max as f32)
    }


    fn xy_to_pixel(xy: f32, max: usize) -> usize {
        (xy * (max as f32) - 0.5).round() as usize
    }

    pub fn fill_rect(&mut self, pos: &Vec2, size: &Vec2, color: &ExternalType, blend_mode: BlendMode) {
        let x_0 = Self::xy_to_pixel(pos.x().clamp(0.0, 1.0), self.width);
        let x_1: usize = Self::xy_to_pixel((pos.x() + size.x()).clamp(0.0, 1.0), self.width);
        let y_0 = Self::xy_to_pixel(pos.y().clamp(0.0, 1.0), self.height);
        let y_1: usize = Self::xy_to_pixel((pos.y() + size.y()).clamp(0.0, 1.0), self.height);
        self.pixels.par_chunks_mut(self.width)
            .skip(y_0)
            .take(y_1 + 1 - y_0)
            .for_each(| chunk| {
                chunk.par_iter_mut()
                    .skip(x_0)
                    .take(x_1 + 1 - x_0)
                    .for_each(|pixel| {
                        BezierCanvas::par_set_pixel(pixel, color, blend_mode);
                    })
            });
    }
    pub fn fill_oval(&mut self, pos: &Vec2, size: &Vec2, color: &ExternalType, blend_mode: BlendMode) {
        let x_0 = Self::xy_to_pixel((pos.x() - size.x()).clamp(0.0, 1.0), self.width);
        let x_1: usize = Self::xy_to_pixel((pos.x() + size.x()).clamp(0.0, 1.0), self.width);
        let y_0 = Self::xy_to_pixel((pos.y() - size.y()).clamp(0.0, 1.0), self.height);
        let y_1: usize = Self::xy_to_pixel((pos.y() + size.y()).clamp(0.0, 1.0), self.height);
        
        let w2 = size.x() * size.x();
        let h2 = size.y() * size.y();
        self.pixels.par_chunks_mut(self.width)
            .skip(y_0)
            .take(y_1 + 1 - y_0)
            .enumerate()
            .for_each(|(i, chunk)| {
                let y = i + y_0;
                let rel_y = Self::pixel_to_xy(y, self.height) - pos.y();
                let y2 = rel_y * rel_y;
                chunk.par_iter_mut()
                    .skip(x_0)
                    .take(x_1 + 1 - x_0)
                    .enumerate()
                    .for_each(|(j, pixel)| {
                        let x = j + x_0;
                        let rel_x = Self::pixel_to_xy(x, self.width) - pos.x();
                        let x2 = rel_x * rel_x;
                        if x2 / w2 + y2 / h2 <= 1f32 {
                            BezierCanvas::par_set_pixel(pixel, color, blend_mode);
                        }
                    })
            });
    }

    pub fn fill_circle(&mut self, pos: &Vec2, radius: f32, color: &ExternalType, blend_mode: BlendMode) {
        self.fill_oval(pos, &Vec2 {v: [radius, radius]}, color, blend_mode);
    }

    fn stroke_line_gentle(&mut self, pos0: &Vec2, pos1: &Vec2, color: &ExternalType, blend_mode: BlendMode) {
        // with abs(slope) < 1
        let x_0 = Self::xy_to_pixel(pos0.x().clamp(0.0, pos1.x()), self.width);
        let x_1: usize = Self::xy_to_pixel(pos0.x().clamp(pos1.x(), 1.0), self.width);
        for x in x_0..=x_1 {
            let y = (pos1.y() - pos0.y()) / (pos1.x() - pos0.x()) * Self::pixel_to_xy(x, self.width) +
             (pos0.y() - (pos1.y() - pos0.y()) / (pos1.x() - pos0.x()) * pos0.x());
            if y >= 0.0f32 && y <= 1.0f32 {
                self.set_pixel(x, Self::xy_to_pixel(y, self.height), color, blend_mode);
            }
        }
    }
    fn stroke_line_steep(&mut self, pos0: &Vec2, pos1: &Vec2, color: &ExternalType, blend_mode: BlendMode) {
        // with abs(slope) < 1

        let y_0 = Self::xy_to_pixel(pos0.y().clamp(0.0, pos1.y()), self.height);
        let y_1: usize = Self::xy_to_pixel(pos0.y().clamp(pos1.y(), 1.0), self.height);
        for y in y_0..=y_1 {
            let x = (pos1.x() - pos0.x()) / (pos1.y() - pos0.y()) * Self::pixel_to_xy(y, self.height) +
             (pos0.x() - (pos1.x() - pos0.x()) / (pos1.y() - pos0.y()) * pos0.y());
            if x >= 0.0f32 && x <=1.0f32 {
                self.set_pixel(Self::xy_to_pixel(x, self.width), y, color, blend_mode);
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

