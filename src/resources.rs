use marmalade::{
    audio::{self, Audio},
    font::{self, Font},
    image,
    render::canvas2d::{Canvas2d, TextureRect},
};

pub struct Assets {}

async fn load_texture(canvas: &mut Canvas2d, bytes: &[u8]) -> TextureRect {
    canvas.create_texture(&image::from_bytes(bytes).await)
}

impl Assets {
    pub async fn load(canvas: &mut Canvas2d) -> Self {
        Self {}
    }
}
