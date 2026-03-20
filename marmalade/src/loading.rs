use glam::Vec2;

use crate::{
    image,
    render::{
        canvas2d::{Canvas2d, DrawTarget2d},
        color,
    },
};

pub struct LoadingContext {}

pub async fn loading<T, O: Future<Output = T>, F: FnOnce(&mut LoadingContext) -> O>(
    canvas: &mut Canvas2d,
    f: F,
) -> T {
    let marmalade_logo_img =
        image::from_bytes(include_bytes!("../resources/images/banner.png")).await;

    let marmalade_logo_texture = canvas.create_texture(&marmalade_logo_img);

    let aspect_ratio = marmalade_logo_img.width() as f32 / marmalade_logo_img.height() as f32;

    canvas.fit_screen();
    canvas.clear(color::rgb(0., 0., 0.));
    canvas.camera_view_ratio(Vec2::ZERO, 1., aspect_ratio);

    canvas.draw_rect(
        Vec2::new(-1., -1. / aspect_ratio),
        Vec2::new(2., 2. / aspect_ratio),
        color::WHITE,
        &marmalade_logo_texture,
    );

    canvas.flush();

    let mut ctx = LoadingContext {};

    f(&mut ctx).await
}
