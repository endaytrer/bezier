use num::Zero;
pub mod blend;
pub trait InternalColorType: Zero + Clone + Copy {}
impl InternalColorType for u8 {}
impl InternalColorType for u16 {}
impl InternalColorType for u32 {}

pub trait ColorType<T: InternalColorType>: Clone + Copy + Eq + PartialEq + blend::Blend<T> {
    fn from_value(raw: T) -> Self;
    fn get_value(&self) -> T;
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
    fn get_value(&self) -> u8 {
        self.a
    }
}
impl ColorType<u8> for R {
    fn from_value(raw: u8) -> Self {
        Self {r: raw}
    }
    fn get_value(&self) -> u8 {
        self.r
    }
}

impl ColorType<u16> for RA {
    fn from_value(raw: u16) -> Self {
        Self { r: ((raw >> 8) & 0xff) as u8, a: (raw & 0xff) as u8 }
    }
    fn get_value(&self) -> u16 {
        (self.a as u16) | ((self.r as u16) << 8)
    }
}

impl ColorType<u32> for RGB {
    fn from_value(raw: u32) -> Self {
        Self { r: ((raw >> 8) & 0xff) as u8, g: ((raw >> 16) & 0xff) as u8, b: ((raw >> 24) & 0xff) as u8 }
    }
    fn get_value(&self) -> u32 {
        ((self.r as u32) << 8) | ((self.g as u32) << 16) | ((self.b as u32) << 24)
    }
}

impl ColorType<u32> for RGBA {
    fn from_value(raw: u32) -> Self {
        Self { a: (raw & 0xff) as u8, r: ((raw >> 8) & 0xff) as u8, g: ((raw >> 16) & 0xff) as u8, b: ((raw >> 24) & 0xff) as u8 }
    }
    fn get_value(&self) -> u32 {
        (self.a as u32) | ((self.r as u32) << 8) | ((self.g as u32) << 16) | ((self.b as u32) << 24)
    }
}
