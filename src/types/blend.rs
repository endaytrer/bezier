use super::*;

#[derive(Copy, Clone)]
pub enum BlendMode {
    Override,
    Alpha,
    Multiply,
    Screen
}
pub trait Blend<InternalType: InternalColorType> {
    fn blend(&self, bg: InternalType, blend_mode: BlendMode) -> InternalType;
}


impl Blend<u8> for A {
    fn blend(&self, bg: u8, blend_mode: BlendMode) -> u8 {
        match blend_mode {
            BlendMode::Override => self.get_value(),
            BlendMode::Alpha => {
                let val = (self.get_value() as f32) / 255f32;
                (255f32 - (255f32 - bg as f32) * (1f32 - val)) as u8
            }
            BlendMode::Multiply => {
                let bg_val = (bg as f32) / 255f32;
                let val = (self.get_value() as f32) / 255f32;
                ((bg_val * val) * 255f32) as u8
            },
            BlendMode::Screen => {
                let bg_val = (bg as f32) / 255f32;
                let val = (self.get_value() as f32) / 255f32;
                ((1f32 - (1f32 - bg_val) * (1f32 - val)) * 255f32) as u8
            }
        }
    }
}

impl Blend<u8> for R {
    fn blend(&self, bg: u8, blend_mode: BlendMode) -> u8 {
        match blend_mode {
            BlendMode::Override => self.get_value(),
            BlendMode::Multiply => {
                let bg_val = (bg as f32) / 255f32;
                let val = (self.get_value() as f32) / 255f32;
                ((bg_val * val) * 255f32) as u8
            },
            BlendMode::Screen => {
                let bg_val = (bg as f32) / 255f32;
                let val = (self.get_value() as f32) / 255f32;
                ((1f32 - (1f32 - bg_val) * (1f32 - val)) * 255f32) as u8
            },
            _ => panic!("Cannot use this blend mode")
        }
    }
}

impl Blend<u16> for RA {
    fn blend(&self, bg: u16, blend_mode: BlendMode) -> u16 {
        match blend_mode {
            BlendMode::Override => self.get_value(),
            BlendMode::Alpha => {
                let bg_ext = RA::from_value(bg);
                let alpha = (self.a as f32) / 255f32;
                let ext = RA {
                    r: ((bg_ext.r as f32) * (1f32 - alpha) + (self.r as f32) * alpha) as u8,
                    a: (255f32 - (255f32 - bg_ext.a as f32) * (1f32 - alpha)) as u8,
                };
                ext.get_value()
            }
            BlendMode::Multiply => {
                let bg_ext = RA::from_value(bg);
                let ext = RA {
                    r: (((bg_ext.r as f32) / 255f32) * ((self.r as f32) / 255f32) * 255f32) as u8,
                    a: (((bg_ext.a as f32) / 255f32) * ((self.a as f32) / 255f32) * 255f32) as u8,
                };
                ext.get_value()
            },
            BlendMode::Screen => {
                let bg_ext = RA::from_value(bg);
                let ext = RA {
                    r: ((1f32 - (1f32 - (bg_ext.r as f32) / 255f32) * (1f32 - (self.r as f32) / 255f32)) * 255f32) as u8,
                    a: ((1f32 - (1f32 - (bg_ext.a as f32) / 255f32) * (1f32 - (self.a as f32) / 255f32)) * 255f32) as u8,
                };
                ext.get_value()
            }
        }
    }
}

impl Blend<u32> for RGB {
    fn blend(&self, bg: u32, blend_mode: BlendMode) -> u32 {
        match blend_mode {
            BlendMode::Override => self.get_value(),
            BlendMode::Multiply => {
                let bg_ext = RGB::from_value(bg);
                let ext = RGB {
                    r: (((bg_ext.r as f32) / 255f32) * ((self.r as f32) / 255f32) * 255f32) as u8,
                    g: (((bg_ext.g as f32) / 255f32) * ((self.g as f32) / 255f32) * 255f32) as u8,
                    b: (((bg_ext.b as f32) / 255f32) * ((self.b as f32) / 255f32) * 255f32) as u8,
                };
                ext.get_value()
            },
            BlendMode::Screen => {
                let bg_ext = RGB::from_value(bg);
                let ext = RGB {
                    r: ((1f32 - (1f32 - (bg_ext.r as f32) / 255f32) * (1f32 - (self.r as f32) / 255f32)) * 255f32) as u8,
                    g: ((1f32 - (1f32 - (bg_ext.g as f32) / 255f32) * (1f32 - (self.g as f32) / 255f32)) * 255f32) as u8,
                    b: ((1f32 - (1f32 - (bg_ext.b as f32) / 255f32) * (1f32 - (self.b as f32) / 255f32)) * 255f32) as u8,
                };
                ext.get_value()
            },
            _ => panic!("Cannot use this blend mode")
        }
    }
}

impl Blend<u32> for RGBA {
    fn blend(&self, bg: u32, blend_mode: BlendMode) -> u32 {
        match blend_mode {
            BlendMode::Override => self.get_value(),
            BlendMode::Alpha => {
                let bg_ext = RGBA::from_value(bg);
                let alpha = (self.a as f32) / 255f32;
                let ext = RGBA {
                    r: ((bg_ext.r as f32) * (1f32 - alpha) + (self.r as f32) * alpha) as u8,
                    g: ((bg_ext.g as f32) * (1f32 - alpha) + (self.g as f32) * alpha) as u8,
                    b: ((bg_ext.b as f32) * (1f32 - alpha) + (self.b as f32) * alpha) as u8,
                    a: (255f32 - (255f32 - bg_ext.a as f32) * (1f32 - alpha)) as u8,
                };
                ext.get_value()
            }
            BlendMode::Multiply => {
                let bg_ext = RGBA::from_value(bg);
                let ext = RGBA {
                    r: (((bg_ext.r as f32) / 255f32) * ((self.r as f32) / 255f32) * 255f32) as u8,
                    g: (((bg_ext.g as f32) / 255f32) * ((self.g as f32) / 255f32) * 255f32) as u8,
                    b: (((bg_ext.b as f32) / 255f32) * ((self.b as f32) / 255f32) * 255f32) as u8,
                    a: (((bg_ext.a as f32) / 255f32) * ((self.a as f32) / 255f32) * 255f32) as u8,
                };
                ext.get_value()
            },
            BlendMode::Screen => {
                let bg_ext = RGBA::from_value(bg);
                let ext = RGBA {
                    r: ((1f32 - (1f32 - (bg_ext.r as f32) / 255f32) * (1f32 - (self.r as f32) / 255f32)) * 255f32) as u8,
                    g: ((1f32 - (1f32 - (bg_ext.g as f32) / 255f32) * (1f32 - (self.g as f32) / 255f32)) * 255f32) as u8,
                    b: ((1f32 - (1f32 - (bg_ext.b as f32) / 255f32) * (1f32 - (self.b as f32) / 255f32)) * 255f32) as u8,
                    a: ((1f32 - (1f32 - (bg_ext.a as f32) / 255f32) * (1f32 - (self.a as f32) / 255f32)) * 255f32) as u8,
                };
                ext.get_value()
            }
        }
    }
}