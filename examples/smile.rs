extern crate ecsgui;

use ecsgui::{
    specs::{World, Builder, DispatcherBuilder},
    specs_hierarchy::HierarchySystem,
    component::{Position, Size, Stroke, Fill, Color, Shape, ShapeCommand::*, Transform, Parent},
    entity::{Cuboid, Cylinder, Path},
};

fn main() {
    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new()
        .with(HierarchySystem::<Parent>::new(), "hierarchy_system", &[])
        .build();
    dispatcher.setup(&mut world.res);

    let _rect = Cuboid::default()
        .with_center(Position { x: 15.0, y: 15.0, z: 0.0 })
        .with_size( Size { width: 30.0, height: 40.0, length: 0.0 })
        .with_fill( Fill { color: Some(Color::Black) })
        .with_stroke(Stroke { color: Some(Color::Red), ..Stroke::default() })
        .build_entity(&mut world);

    let face = Cylinder::default()
        .with_center(Position { x: 15.0, y: 15.0, z: 0.0 })
        .with_radius(10.0)
        .with_fill(Fill { color: Some(Color::Yellow) })
        .build_entity(&mut world);

    let eye_left = Cylinder::default()
        .with_center(Position { x: 12.0, y: 15.0, z: 0.0 })
        .with_radius(1.5)
        .with_fill(Fill { color: Some(Color::Black) })
        .build_entity(&mut world);

    let eye_right = Cylinder::default()
        .with_center(Position { x: 17.0, y: 15.0, z: 0.0 })
        .with_radius(1.5)
        .with_fill(Fill { color: Some(Color::Black) })
        .build_entity(&mut world);

    let mouth = Path::default()
        .with_shape(Shape(vec![Move([10.0, 19.0].into()), Line([15.0, 23.0].into()), Line([20.0, 19.0].into())]))
        .with_stroke(Stroke { color: Some(Color::Black), width: 2.0 })
        .build_entity(&mut world);

    let group = world.create_entity()
        .with(Transform::Translate([0.0, 0.5, 0.0].into()))
        .build();

    {
        let mut parents = world.write_storage::<Parent>();
        parents.insert(face, Parent(group)).unwrap();
        parents.insert(eye_left, Parent(group)).unwrap();
        parents.insert(eye_right, Parent(group)).unwrap();
        parents.insert(mouth, Parent(group)).unwrap();
    }

    dispatcher.dispatch(&mut world.res);
}