use specs::{Component, World};
use specs::Entity;
use specs::storage::{DenseVecStorage, FlaggedStorage};
use specs_hierarchy::{Parent as SHParent};
use types::Real;

#[derive(Component, Debug, Default, Clone)]
#[storage(DenseVecStorage)]
pub struct Position {
    pub x: Real,
    pub y: Real,
    pub z: Real,
}

impl From<(Real, Real, Real)> for Position {
    fn from(xyz: (Real, Real, Real)) -> Self {
        Position { x: xyz.0, y: xyz.1, z: xyz.2 }
    }
}

impl From<(Real, Real)> for Position {
    fn from(xy: (Real, Real)) -> Self {
        Position { x: xy.0, y: xy.1, z: Default::default() }
    }
}

impl From<Real> for Position {
    fn from(x: Real) -> Self {
        Position { x, y: Default::default(), z: Default::default() }
    }
}

impl From<[Real; 3]> for Position {
    fn from(xyz: [Real; 3]) -> Self {
        Position { x: xyz[0], y: xyz[1], z: xyz[2] }
    }
}

impl From<[Real; 2]> for Position {
    fn from(xy: [Real; 2]) -> Self {
        Position { x: xy[0], y: xy[1], z: Default::default() }
    }
}

impl From<[Real; 1]> for Position {
    fn from(x: [Real; 1]) -> Self {
        Position { x: x[0], y: Default::default(), z: Default::default() }
    }
}

#[derive(Component, Debug, Default, Clone)]
#[storage(DenseVecStorage)]
pub struct Size {
    pub length: Real,
    pub width: Real,
    pub height: Real,
}

#[derive(Component, Debug, Default, Copy, Clone)]
#[storage(DenseVecStorage)]
pub struct RealValue(pub Real);

#[derive(Component, Debug, Default, Copy, Clone)]
#[storage(DenseVecStorage)]
pub struct Radius(pub Real);

#[derive(Component, Debug, Default, Copy, Clone)]
#[storage(DenseVecStorage)]
pub struct Depth(pub Real);

impl From<Real> for RealValue {
    fn from(value: Real) -> Self {
        RealValue(value)
    }
}

#[derive(Debug, Clone)]
pub enum Color {
    Red,
    Yellow,
    Black,
}

#[derive(Component, Debug, Default, Clone)]
#[storage(DenseVecStorage)]
pub struct Fill {
    pub color: Option<Color>,
}

#[derive(Component, Debug, Default, Clone)]
#[storage(DenseVecStorage)]
pub struct Stroke {
    pub color: Option<Color>,
    pub width: Real,
}

#[derive(Debug, Clone)]
pub enum ShapeCommand {
    Move(Position),
    MoveRel(Position),
    Line(Position),
    LineRel(Position),
    LineAlonX(Real),
    LineAlonXRel(Real),
    LineAlonY(Real),
    LineAlonYRel(Real),
    LineAlonZ(Real),
    LineAlonZRel(Real),
    Close,
}

#[derive(Component, Debug, Default, Clone)]
#[storage(DenseVecStorage)]
pub struct Shape(pub Vec<ShapeCommand>);


#[derive(Component, Debug, Clone)]
#[storage(DenseVecStorage)]
pub enum Transform {
    Translate(Position),
    None,
}

impl Default for Transform {
    fn default() -> Self {
        Transform::None
    }
}


#[derive(Debug, Clone)]
pub struct Parent(pub Entity);

impl Component for Parent {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

impl SHParent for Parent {
    fn parent_entity(&self) -> Entity {
        self.0
    }
}

pub fn register_all_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Size>();
    world.register::<RealValue>();
    world.register::<Radius>();
    world.register::<Depth>();
    world.register::<Stroke>();
    world.register::<Fill>();
    world.register::<Shape>();
    world.register::<Transform>();
    world.register::<Parent>();
}