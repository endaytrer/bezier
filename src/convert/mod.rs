use std::{path::Path, fs::File, io::BufWriter};
use crate::{BezierCanvas, types::{RGBA, RGB}};


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
impl BezierCanvas<u32, RGBA> {
    pub fn export_png(&self, img_path: &str) {
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
}
impl BezierCanvas<u32, RGB> {
    pub fn export_png(&self, img_path: &str) {
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
}