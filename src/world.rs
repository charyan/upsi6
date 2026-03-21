use std::{cell::RefCell, rc::Rc};

use glam::Vec2;

pub struct Resource {
    pub radius: f32,
    pub pos: Vec2,
}

impl Resource {
    pub fn new(radius: f32, pos: Vec2) -> Resource {
        Resource { radius, pos }
    }
}

pub struct World {
    pub selected: Option<Rc<RefCell<Resource>>>,
    pub view_radius: f32,
    pub resources: Vec<Rc<RefCell<Resource>>>,
}

impl World {
    pub fn new() -> World {
        let mut resources = Vec::new();

        for x in 0..3 {
            for y in 0..3 {
                resources.push(Rc::new(RefCell::new(Resource::new(
                    0.5,
                    Vec2::new((x - 1) as f32 * 2.0, (y - 1) as f32 * 2.0),
                ))));
            }
        }

        World {
            selected: None,
            view_radius: 4.,
            resources,
        }
    }

    pub fn tick(&mut self) {
        self.view_radius *= 1.00
    }
}
