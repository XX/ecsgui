pub extern crate specs;
pub extern crate specs_hierarchy;
#[macro_use]
extern crate specs_derive;

pub mod types;
pub mod component;
pub mod entity;

use specs::World;
use component::register_all_components;

pub fn create_world() -> World {
    let mut world = World::new();
    register_all_components(&mut world);
    world
}
