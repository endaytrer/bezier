use super::*;

#[derive(Copy, Clone)]
pub enum BlendMode {
    Override,
    Alpha,
    Multiply,
    Screen
}
impl BlendMode {
    pub fn blend<InternalType: InternalColorType, ExternalType: ColorType<InternalType>>(&self, bg: InternalType, fg: &ExternalType) -> InternalType {
        match self {
            BlendMode::Override => fg.to_value(),
            BlendMode::Alpha => {
                let fg_vec = fg.to_vec4();
                let bg_vec = ExternalType::from_value(bg).to_vec4();
                let alpha = fg_vec.w();
                let mut ans = bg_vec * (1f32 - alpha) + fg_vec * alpha;
                ans.v[3] = 1.0 - (1.0 - bg_vec.w()) * (1.0 - fg_vec.w());
                ExternalType::from_vec4(ans).to_value()
            },
            BlendMode::Multiply => {
                let fg_vec = fg.to_vec4();
                let bg_vec = ExternalType::from_value(bg).to_vec4();
                let ans = fg_vec.star(&bg_vec);
                ExternalType::from_vec4(ans).to_value()
            },
            BlendMode::Screen => {
                let full = Vec4::new(1.0, 1.0, 1.0, 1.0);
                let fg_vec = full - fg.to_vec4();
                let bg_vec = full - ExternalType::from_value(bg).to_vec4();
                let ans = full - fg_vec.star(&bg_vec);
                ExternalType::from_vec4(ans).to_value()
            },
        }
    }
}