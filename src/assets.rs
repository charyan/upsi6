use marmalade::{
    audio::{self, Audio},
    font::{self, Font},
    image,
    render::canvas2d::{Canvas2d, TextureRect},
};

pub struct Assets {
    pub l1: TextureRect, // 3840 x 2160
    pub l1_can: TextureRect,
    pub l1_chair: TextureRect,
    pub l1_computer: TextureRect,
    pub l1_desk: TextureRect,
    pub l1_trash: TextureRect,
    pub l1_paper: TextureRect,

    pub l2: TextureRect,
    pub l2_car: TextureRect,
    pub l2_bench: TextureRect,
    pub l2_letterbox: TextureRect,
    pub l2_light: TextureRect,
    pub l2_manholecover: TextureRect,
    pub l2_truck: TextureRect,

    pub l3: TextureRect,
    pub l3_l2_object: TextureRect,
    pub l3_airport: TextureRect,
    pub l3_boat: TextureRect,
    pub l3_bridge: TextureRect,
    pub l3_building: TextureRect,
    pub l3_container: TextureRect,
    pub l3_cow: TextureRect,
    pub l3_crane: TextureRect,
    pub l3_hotairbaloon: TextureRect,
    pub l3_house: TextureRect,
    pub l3_object: TextureRect,
    pub l3_tree: TextureRect,
    pub l3_watertower: TextureRect,
    pub l3_whale: TextureRect,

    pub l4: TextureRect,
    pub l4_comet: TextureRect,
    pub l4_helmet: TextureRect,
    pub l4_milkyway: TextureRect,
    pub l4_moon: TextureRect,
    pub l4_sat: TextureRect,
    pub l4_star: TextureRect,
    pub l4_sun: TextureRect,

    pub font: Font,
    pub s1: Audio,
    pub shredder_wheel: TextureRect,
    pub shredder_box: TextureRect,
    pub shredder_panel_ok: TextureRect,
    pub shredder_panel_nok_1: TextureRect,
    pub shredder_panel_nok_2: TextureRect,

    pub gui_key_up: TextureRect,
    pub gui_key_down: TextureRect,
    pub gui_gauge: TextureRect,
    pub gui_window: TextureRect,

    pub energy: TextureRect,
    pub gears: TextureRect,
    pub oil: TextureRect,

    pub earth_resource: TextureRect,

    pub music_act: [Audio; 4],
    pub shreder_sound: Audio,
}

async fn load_texture(canvas: &mut Canvas2d, bytes: &[u8]) -> TextureRect {
    canvas.create_texture(&image::from_bytes(bytes).await)
}

impl Assets {
    pub async fn load(canvas: &mut Canvas2d) -> Self {
        Self {
            // l1
            l1: load_texture(canvas, include_bytes!("../assets/L1_base.png")).await,
            l1_can: load_texture(canvas, include_bytes!("../assets/L1_can.png")).await,
            l1_chair: load_texture(canvas, include_bytes!("../assets/L1_chair.png")).await,
            l1_computer: load_texture(canvas, include_bytes!("../assets/L1_computer.png")).await,
            l1_desk: load_texture(canvas, include_bytes!("../assets/L1_desk.png")).await,
            l1_trash: load_texture(canvas, include_bytes!("../assets/L1_trash.png")).await,
            l1_paper: load_texture(canvas, include_bytes!("../assets/L1_paper.png")).await,

            // l2
            l2: load_texture(canvas, include_bytes!("../assets/L2_base.png")).await,
            l2_car: load_texture(canvas, include_bytes!("../assets/L2_car.png")).await,
            l2_bench: load_texture(canvas, include_bytes!("../assets/L2_bench.png")).await,
            l2_letterbox: load_texture(canvas, include_bytes!("../assets/L2_letterbox.png")).await,
            l2_light: load_texture(canvas, include_bytes!("../assets/L2_light.png")).await,
            l2_manholecover: load_texture(canvas, include_bytes!("../assets/L2_manholecover.png"))
                .await,
            l2_truck: load_texture(canvas, include_bytes!("../assets/L2_truck.png")).await,

            // l3
            l3: load_texture(canvas, include_bytes!("../assets/L3_base.png")).await,
            l3_l2_object: load_texture(canvas, include_bytes!("../assets/L2_object.png")).await,
            l3_airport: load_texture(canvas, include_bytes!("../assets/L3_airport.png")).await,
            l3_boat: load_texture(canvas, include_bytes!("../assets/L3_boat.png")).await,
            l3_bridge: load_texture(canvas, include_bytes!("../assets/L3_bridge.png")).await,
            l3_building: load_texture(canvas, include_bytes!("../assets/L3_building.png")).await,
            l3_container: load_texture(canvas, include_bytes!("../assets/L3_container.png")).await,
            l3_cow: load_texture(canvas, include_bytes!("../assets/L3_cow.png")).await,
            l3_crane: load_texture(canvas, include_bytes!("../assets/L3_crane.png")).await,
            l3_hotairbaloon: load_texture(canvas, include_bytes!("../assets/L3_hotairbaloon.png"))
                .await,
            l3_house: load_texture(canvas, include_bytes!("../assets/L3_house.png")).await,
            l3_object: load_texture(canvas, include_bytes!("../assets/L3_object.png")).await,
            l3_tree: load_texture(canvas, include_bytes!("../assets/L3_tree.png")).await,
            l3_watertower: load_texture(canvas, include_bytes!("../assets/L3_watertower.png"))
                .await,
            l3_whale: load_texture(canvas, include_bytes!("../assets/L3_whale.png")).await,

            // l4
            l4: load_texture(canvas, include_bytes!("../assets/L4_base.png")).await,
            l4_comet: load_texture(canvas, include_bytes!("../assets/L4_comet.png")).await,
            l4_helmet: load_texture(canvas, include_bytes!("../assets/L4_helmet.png")).await,
            l4_milkyway: load_texture(canvas, include_bytes!("../assets/L4_milkyway.png")).await,
            l4_moon: load_texture(canvas, include_bytes!("../assets/L4_moon.png")).await,
            l4_sat: load_texture(canvas, include_bytes!("../assets/L4_sat.png")).await,
            l4_star: load_texture(canvas, include_bytes!("../assets/L4_star.png")).await,
            l4_sun: load_texture(canvas, include_bytes!("../assets/L4_sun.png")).await,

            font: Font::new(font::MONOGRAM),
            s1: audio::from_bytes(include_bytes!(
                "../ressources/audio/effects/shred/Destruction_heavy_metal_1.mp3"
            ))
            .await,

            // shredder
            shredder_wheel: load_texture(canvas, include_bytes!("../assets/shredder_wheel.png"))
                .await,
            shredder_box: load_texture(canvas, include_bytes!("../assets/shredder_box.png")).await,
            shredder_panel_ok: load_texture(
                canvas,
                include_bytes!("../assets/shredder_panel_ok.png"),
            )
            .await,
            shredder_panel_nok_1: load_texture(
                canvas,
                include_bytes!("../assets/shredder_panel_nok_1.png"),
            )
            .await,
            shredder_panel_nok_2: load_texture(
                canvas,
                include_bytes!("../assets/shredder_panel_nok_2.png"),
            )
            .await,

            gui_key_down: load_texture(canvas, include_bytes!("../assets/gui_key_down.png")).await,
            gui_key_up: load_texture(canvas, include_bytes!("../assets/gui_key_up.png")).await,
            gui_gauge: load_texture(canvas, include_bytes!("../assets/gui_gauge.png")).await,
            gui_window: load_texture(canvas, include_bytes!("../assets/gui_window.png")).await,

            energy: load_texture(canvas, include_bytes!("../assets/energy.png")).await,
            gears: load_texture(canvas, include_bytes!("../assets/gears.png")).await,
            oil: load_texture(canvas, include_bytes!("../assets/oil.png")).await,

            earth_resource: load_texture(canvas, include_bytes!("../assets/L3_object.png")).await,

            music_act: [
                audio::from_bytes(include_bytes!("../ressources/audio/music/upsi6-act1.mp3")).await,
                audio::from_bytes(include_bytes!("../ressources/audio/music/upsi6-act2.mp3")).await,
                audio::from_bytes(include_bytes!("../ressources/audio/music/upsi6-act3.mp3")).await,
                audio::from_bytes(include_bytes!("../ressources/audio/music/upsi6-act4.mp3")).await,
            ],
            shreder_sound: audio::from_bytes(include_bytes!(
                "../ressources/audio/effects/active_shreder_sound.mp3"
            ))
            .await,
        }
    }
}
