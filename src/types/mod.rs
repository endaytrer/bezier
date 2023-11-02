use num::Zero;

use crate::linalg::Vec4;
pub mod blend;
pub trait InternalColorType: Sync + Send + Zero + Clone + Copy {}
impl InternalColorType for u8 {}
impl InternalColorType for u16 {}
impl InternalColorType for u32 {}

pub trait ColorType<T: InternalColorType>: Clone + Copy + Eq + PartialEq{
    fn from_value(raw: T) -> Self;
    fn to_value(&self) -> T;
    fn from_vec4(raw: Vec4) -> Self;
    fn to_vec4(&self) -> Vec4;
}
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct A {
    pub a: u8
}
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct R {
    pub r: u8
}
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct RA {
    pub r: u8,
    pub a: u8
}
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl ColorType<u8> for A {
    fn from_value(raw: u8) -> Self {
        Self {a: raw}
    }
    fn to_value(&self) -> u8 {
        self.a
    }

    fn to_vec4(&self) -> Vec4 {
        Vec4::new(0.0, 0.0, 0.0, self.a as f32 / 255f32)
    }

    fn from_vec4(raw: Vec4) -> Self {
        Self {a: (raw.w() * 255f32).round() as u8}
    }
}
impl ColorType<u8> for R {
    fn from_value(raw: u8) -> Self {
        Self {r: raw}
    }
    fn to_value(&self) -> u8 {
        self.r
    }
    fn to_vec4(&self) -> Vec4 {
        Vec4::new(self.r as f32 / 255f32, self.r as f32 / 255f32, self.r as f32 / 255f32, 1.0)
    }
    fn from_vec4(raw: Vec4) -> Self {
        Self {r: ((raw.x() + raw.y() + raw.z()) * 85f32).round() as u8}
    }
}

impl ColorType<u16> for RA {
    fn from_value(raw: u16) -> Self {
        Self { r: ((raw >> 8) & 0xff) as u8, a: (raw & 0xff) as u8 }
    }
    fn to_value(&self) -> u16 {
        (self.a as u16) | ((self.r as u16) << 8)
    }
    fn to_vec4(&self) -> Vec4 {
        Vec4::new(self.r as f32 / 255f32, self.r as f32 / 255f32, self.r as f32 / 255f32, self.a as f32 / 255f32)
    }
    fn from_vec4(raw: Vec4) -> Self {
        Self {r: ((raw.x() + raw.y() + raw.z()) * 85f32).round() as u8, a: (raw.w() * 255f32).round() as u8}
    }
}

impl ColorType<u32> for RGB {
    fn from_value(raw: u32) -> Self {
        Self { r: ((raw >> 8) & 0xff) as u8, g: ((raw >> 16) & 0xff) as u8, b: ((raw >> 24) & 0xff) as u8 }
    }
    fn to_value(&self) -> u32 {
        ((self.r as u32) << 8) | ((self.g as u32) << 16) | ((self.b as u32) << 24)
    }
    fn to_vec4(&self) -> Vec4 {
        Vec4::new(self.r as f32 / 255f32, self.g as f32 / 255f32, self.b as f32 / 255f32, 1.0)
    }

    fn from_vec4(raw: Vec4) -> Self {
        Self {r: (raw.x() * 255f32).round() as u8, g: (raw.y() * 255f32).round() as u8, b: (raw.z() * 255f32).round() as u8}
    }
}

impl ColorType<u32> for RGBA {
    fn from_value(raw: u32) -> Self {
        Self { a: (raw & 0xff) as u8, r: ((raw >> 8) & 0xff) as u8, g: ((raw >> 16) & 0xff) as u8, b: ((raw >> 24) & 0xff) as u8 }
    }
    fn to_value(&self) -> u32 {
        (self.a as u32) | ((self.r as u32) << 8) | ((self.g as u32) << 16) | ((self.b as u32) << 24)
    }
    fn to_vec4(&self) -> Vec4 {
        Vec4::new(self.r as f32 / 255f32, self.g as f32 / 255f32, self.b as f32 / 255f32, self.a as f32 / 255f32)
    }
    fn from_vec4(raw: Vec4) -> Self {
        Self {r: (raw.x() * 255f32).round() as u8, g: (raw.y() * 255f32).round() as u8, b: (raw.z() * 255f32).round() as u8, a: (raw.w() * 255f32).round() as u8}
    }
}
