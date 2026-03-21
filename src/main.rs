use glam::Vec2;
use marmalade::dom_stack;
use marmalade::draw_scheduler;
use marmalade::font;
use marmalade::image;
use marmalade::input;
use marmalade::input::Button;
use marmalade::loading;
use marmalade::render::canvas2d::Canvas2d;
use marmalade::render::canvas2d::DrawTarget2d;
use marmalade::render::color;

use crate::assets::Assets;
use crate::world::World;

mod assets;
mod world;

pub const ASPECT_RATIO: f32 = 16.0 / 9.0;
pub const WORLD_SIZE: Vec2 = Vec2::new(16., 9.);

fn draw_game(canvas: &mut Canvas2d, world: &mut World, assets: &Assets) {
    canvas.camera_view_ratio(Vec2::new(0.0, 0.0), world.view_radius, ASPECT_RATIO);

    let mouse_pos = canvas.screen_to_world_pos(input::mouse_position().as_vec2());

    canvas.draw_rect(
        Vec2::new(-16000., -9000.),
        Vec2::new(32000., 18000.),
        color::WHITE,
        &assets.l4,
    );

    canvas.draw_rect(
        Vec2::new(-1600., -900.),
        Vec2::new(3200., 1800.),
        color::WHITE,
        &assets.l3,
    );

    canvas.draw_rect(
        Vec2::new(-160., -90.),
        Vec2::new(320., 180.),
        color::WHITE,
        &assets.l2,
    );

    canvas.draw_rect(
        Vec2::new(-16., -9.),
        Vec2::new(32., 18.),
        color::WHITE,
        &assets.l1,
    );

    if !input::is_button_down(Button::Left) {
        world.selected = None;
    }

    let mouse_clicked = input::is_button_pressed(Button::Left);

    for resource in &world.resources {
        let r = resource.borrow();

        let radius = r.radius
            * if r.pos.distance(mouse_pos) < r.radius {
                if mouse_clicked {
                    world.selected = Some(resource.clone());
                }
                1.1
            } else {
                1.0
            };

        canvas.draw_regular(
            r.pos,
            radius,
            64,
            color::rgb(1.0, 0.0, 0.0),
            &canvas.white_texture(),
        );
    }

    if let Some(selected) = &world.selected {
        let mut s = selected.borrow_mut();

        let dist = mouse_pos - s.pos;

        s.pos += dist * 0.1;
    }
}

async fn async_main() {
    dom_stack::set_title("UPSI 6");

    let main_canvas = dom_stack::create_full_screen_canvas();

    dom_stack::stack_node(&main_canvas);

    let mut canvas = Canvas2d::new(&main_canvas);

    loading::loading(&mut canvas, |_| async {}).await;

    let assets = Assets::load(&mut canvas).await;

    let image = image::from_bytes(include_bytes!("../marmalade/resources/images/logo.png")).await;

    let image_rect = canvas.create_texture(&image);

    let mut font = font::from_bytes(font::MONOGRAM);

    let mut world = World::new();

    draw_scheduler::set_on_draw(move || {
        world.tick();

        canvas.fit_screen();

        canvas.clear(color::rgb(0., 0., 0.));

        draw_game(&mut canvas, &mut world, &assets);

        canvas.flush();
    });
}

fn main() {
    console_error_panic_hook::set_once();

    wasm_bindgen_futures::spawn_local(async_main());
}
