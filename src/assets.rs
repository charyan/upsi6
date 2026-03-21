use marmalade::{
    audio::{self, Audio},
    font::{self, Font},
    image,
    render::canvas2d::{Canvas2d, TextureRect},
};

pub struct Assets {
    pub l1: TextureRect, // 3840 x 2160
    pub l2: TextureRect,
    pub l3: TextureRect,
    pub l4: TextureRect,
    pub font: Font,
    pub s1: Audio,
    pub shredder_wheel: TextureRect,
    pub shredder_box: TextureRect,
}

async fn load_texture(canvas: &mut Canvas2d, bytes: &[u8]) -> TextureRect {
    canvas.create_texture(&image::from_bytes(bytes).await)
}

impl Assets {
    pub async fn load(canvas: &mut Canvas2d) -> Self {
        Self {
            l1: load_texture(canvas, include_bytes!("../assets/L1_base.png")).await,
            l2: load_texture(canvas, include_bytes!("../assets/L2_base.png")).await,
            l3: load_texture(canvas, include_bytes!("../assets/L3_base.png")).await,
            l4: load_texture(canvas, include_bytes!("../assets/L4_base.png")).await,
            font: Font::new(font::MONOGRAM),
            s1: audio::from_bytes(include_bytes!(
                "../ressources/audio/effects/shred/Destruction_heavy_metal_1.mp3"
            ))
            .await,
            shredder_wheel: load_texture(canvas, include_bytes!("../assets/shredder_wheel.png"))
                .await,
            shredder_box: load_texture(canvas, include_bytes!("../assets/shredder_box.png")).await,
        }
    }
}
