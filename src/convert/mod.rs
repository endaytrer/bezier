use std::{path::Path, fs::File, io::BufWriter};
use crate::{BezierCanvas, types::{RGBA, RGB, ColorType, RA, R, A}, BezierCanvasFactory};

fn init_writer(img_path: &str, width: u32, height: u32, color_type: png::ColorType) -> png::Writer<BufWriter<File>> {
    let path = Path::new(img_path);
    let file = File::create(path).unwrap();

    let w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(color_type);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));     // 1.0 / 2.2, unscaled, but rounded
    let source_chromaticities = png::SourceChromaticities::new(     // Using unscaled instantiation here
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000)
    );
    encoder.set_source_chromaticities(source_chromaticities);
    encoder.write_header().unwrap()
}
pub trait PNGCompatible {
    fn export_png(&self, img_path: &str);
    fn from_png(img_path: &str) -> Self;
}

impl PNGCompatible for BezierCanvas<u32, RGBA> {
    fn export_png(&self, img_path: &str) {
        let ref mut writer = init_writer(img_path, self.width as u32, self.height as u32, png::ColorType::Rgba);

        let mut data: Vec<u8> = vec![0; 4 * self.width * self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                data[(y * self.width + x) * 4] = self.get_pixel(x, y).r;
                data[(y * self.width + x) * 4 + 1] = self.get_pixel(x, y).g;
                data[(y * self.width + x) * 4 + 2] = self.get_pixel(x, y).b;
                data[(y * self.width + x) * 4 + 3] = self.get_pixel(x, y).a;
            }
        }
        writer.write_image_data(&data).unwrap();
    }
    fn from_png(img_path: &str) -> Self {
        let decoder = png::Decoder::new(File::open(img_path).unwrap());
        let mut reader = decoder.read_info().unwrap();
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf).unwrap();
        let bytes = &buf[..info.buffer_size()];
        let mut canvas = BezierCanvasFactory::new().set_size(info.width as usize, info.height as usize).create_canvas();

        match info.color_type {
            png::ColorType::Rgba => {
                for y in 0..canvas.height {
                    for x in 0..canvas.width {
                        canvas.pixels[y * canvas.width + x] = RGBA {
                            r: bytes[(y * canvas.width + x) * 4],
                            g: bytes[(y * canvas.width + x) * 4 + 1],
                            b: bytes[(y * canvas.width + x) * 4 + 2],
                            a: bytes[(y * canvas.width + x) * 4 + 3],
                        }.to_value();
                    }
                }
            }
            png::ColorType::Rgb => {
                for y in 0..canvas.height {
                    for x in 0..canvas.width {
                        canvas.pixels[y * canvas.width + x] = RGBA {
                            r: bytes[(y * canvas.width + x) * 3],
                            g: bytes[(y * canvas.width + x) * 3 + 1],
                            b: bytes[(y * canvas.width + x) * 3 + 2],
                            a: 255,
                        }.to_value();
                    }
                }
            }
            png::ColorType::Grayscale => {
                for y in 0..canvas.height {
                    for x in 0..canvas.width {
                        canvas.pixels[y * canvas.width + x] = RGBA {
                            r: bytes[y * canvas.width + x],
                            g: bytes[y * canvas.width + x],
                            b: bytes[y * canvas.width + x],
                            a: 255,
                        }.to_value();
                    }
                }
            }
            png::ColorType::Indexed => {
                for y in 0..canvas.height {
                    for x in 0..canvas.width {
                        canvas.pixels[y * canvas.width + x] = RGBA {
                            r: 0,
                            g: 0,
                            b: 0,
                            a: bytes[y * canvas.width + x],
                        }.to_value();
                    }
                }
            },
            png::ColorType::GrayscaleAlpha => {
                for y in 0..canvas.height {
                    for x in 0..canvas.width {
                        canvas.pixels[y * canvas.width + x] = RGBA {
                            r: bytes[(y * canvas.width + x) * 2],
                            g: bytes[(y * canvas.width + x) * 2],
                            b: bytes[(y * canvas.width + x) * 2],
                            a: bytes[(y * canvas.width + x) * 2 + 1],
                        }.to_value();
                    }
                }
            },
        }
        canvas
    }
}
impl PNGCompatible for BezierCanvas<u32, RGB> {
    fn export_png(&self, img_path: &str) {
        let ref mut writer = init_writer(img_path, self.width as u32, self.height as u32, png::ColorType::Rgb);

        let mut data: Vec<u8> = vec![0; 3 * self.width * self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                data[(y * self.width + x) * 3] = self.get_pixel(x, y).r;
                data[(y * self.width + x) * 3 + 1] = self.get_pixel(x, y).g;
                data[(y * self.width + x) * 3 + 2] = self.get_pixel(x, y).b;
            }
        }
        writer.write_image_data(&data).unwrap();

    }
    fn from_png(img_path: &str) -> Self {
        let decoder = png::Decoder::new(File::open(img_path).unwrap());
        let mut reader = decoder.read_info().unwrap();
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf).unwrap();
        let bytes = &buf[..info.buffer_size()];
        let mut canvas = BezierCanvasFactory::new().set_size(info.width as usize, info.height as usize).create_canvas();
        
        match info.color_type {
            png::ColorType::Rgb => {
                for y in 0..canvas.height {
                    for x in 0..canvas.width {
                        canvas.pixels[y * canvas.width + x] = RGB {
                            r: bytes[(y * canvas.width + x) * 3],
                            g: bytes[(y * canvas.width + x) * 3 + 1],
                            b: bytes[(y * canvas.width + x) * 3 + 2],
                        }.to_value();
                    }
                }
            }
            png::ColorType::Grayscale => {
                for y in 0..canvas.height {
                    for x in 0..canvas.width {
                        canvas.pixels[y * canvas.width + x] = RGB {
                            r: bytes[y * canvas.width + x],
                            g: bytes[y * canvas.width + x],
                            b: bytes[y * canvas.width + x],
                        }.to_value();
                    }
                }
            }
            _ => panic!("Incompatible color type"),
        }
        canvas
    }
}
impl PNGCompatible for BezierCanvas<u16, RA> {
    fn export_png(&self, img_path: &str) {
        let ref mut writer = init_writer(img_path, self.width as u32, self.height as u32, png::ColorType::GrayscaleAlpha);

        let mut data: Vec<u8> = vec![0; 3 * self.width * self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                data[(y * self.width + x) * 2] = self.get_pixel(x, y).r;
                data[(y * self.width + x) * 2 + 1] = self.get_pixel(x, y).a;
            }
        }
        writer.write_image_data(&data).unwrap();

    }
    fn from_png(img_path: &str) -> Self {
        let decoder = png::Decoder::new(File::open(img_path).unwrap());
        let mut reader = decoder.read_info().unwrap();
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf).unwrap();
        let bytes = &buf[..info.buffer_size()];
        let mut canvas = BezierCanvasFactory::new().set_size(info.width as usize, info.height as usize).create_canvas();
        
        match info.color_type {
            png::ColorType::GrayscaleAlpha => {
                for y in 0..canvas.height {
                    for x in 0..canvas.width {
                        canvas.pixels[y * canvas.width + x] = RA {
                            r: bytes[(y * canvas.width + x) * 2],
                            a: bytes[(y * canvas.width + x) * 2 + 1],
                        }.to_value();
                    }
                }
            }
            png::ColorType::Grayscale => {
                for y in 0..canvas.height {
                    for x in 0..canvas.width {
                        canvas.pixels[y * canvas.width + x] = RA {
                            r: bytes[y * canvas.width + x],
                            a: 255,
                        }.to_value();
                    }
                }
            }
            png::ColorType::Indexed => {
                for y in 0..canvas.height {
                    for x in 0..canvas.width {
                        canvas.pixels[y * canvas.width + x] = RA {
                            r: 0,
                            a: bytes[y * canvas.width + x],
                        }.to_value();
                    }
                }
            }
            _ => panic!("Incompatible color type"),
        }
        canvas
    }
}
impl PNGCompatible for BezierCanvas<u8, R> {
    fn export_png(&self, img_path: &str) {
        let ref mut writer = init_writer(img_path, self.width as u32, self.height as u32, png::ColorType::Grayscale);

        let mut data: Vec<u8> = vec![0; 3 * self.width * self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                data[y * self.width + x] = self.get_pixel(x, y).r;
            }
        }
        writer.write_image_data(&data).unwrap();

    }
    fn from_png(img_path: &str) -> Self {
        let decoder = png::Decoder::new(File::open(img_path).unwrap());
        let mut reader = decoder.read_info().unwrap();
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf).unwrap();
        let bytes = &buf[..info.buffer_size()];
        let mut canvas = BezierCanvasFactory::new().set_size(info.width as usize, info.height as usize).create_canvas();
        
        match info.color_type {
            png::ColorType::Grayscale => {
                for y in 0..canvas.height {
                    for x in 0..canvas.width {
                        canvas.pixels[y * canvas.width + x] = R {
                            r: bytes[y * canvas.width + x],
                        }.to_value();
                    }
                }
            }
            _ => panic!("Incompatible color type"),
        }
        canvas
    }
}
impl PNGCompatible for BezierCanvas<u8, A> {
    fn export_png(&self, img_path: &str) {
        let ref mut writer = init_writer(img_path, self.width as u32, self.height as u32, png::ColorType::Indexed);

        let mut data: Vec<u8> = vec![0; 3 * self.width * self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                data[y * self.width + x] = self.get_pixel(x, y).a;
            }
        }
        writer.write_image_data(&data).unwrap();

    }
    fn from_png(img_path: &str) -> Self {
        let decoder = png::Decoder::new(File::open(img_path).unwrap());
        let mut reader = decoder.read_info().unwrap();
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf).unwrap();
        let bytes = &buf[..info.buffer_size()];
        let mut canvas = BezierCanvasFactory::new().set_size(info.width as usize, info.height as usize).create_canvas();
        
        match info.color_type {
            png::ColorType::Indexed => {
                for y in 0..canvas.height {
                    for x in 0..canvas.width {
                        canvas.pixels[y * canvas.width + x] = A {
                            a: bytes[y * canvas.width + x],
                        }.to_value();
                    }
                }
            }
            _ => panic!("Incompatible color type"),
        }
        canvas
    }
}