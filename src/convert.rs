
pub trait PNGCompatible {
    fn export_png(&self, img_path: &str);
    fn from_png(img_path: &str) -> Self;
}