use specs::{Entity, Builder, World};
use component::{Position, Size, Stroke, Fill};

#[derive(Debug, Default, Clone)]
pub struct Cuboid {
    center: Position,
    size: Size,
    stroke: Option<Stroke>,
    fill: Option<Fill>,
}

impl Cuboid {
    pub fn with_center(mut self, center: Position) -> Self {
        self.center = center;
        self
    }

    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn with_stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = Some(stroke);
        self
    }

    pub fn with_fill(mut self, fill: Fill) -> Self {
        self.fill = Some(fill);
        self
    }

    pub fn build_entity(self, world: &mut World) -> Entity {
        let Cuboid { center, size, stroke, fill } = self;
        let mut entity = world.create_entity()
            .with(center)
            .with(size);

        if let Some(stroke) = stroke {
            entity = entity.with(stroke);
        }
        if let Some(fill) = fill {
            entity = entity.with(fill);
        }

        entity.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_rectangle() {
        let mut world = ::create_world();
        let cube = Cuboid::default()
            .with_center(Position { x: 1.0, y: 1.0, z: 0.0 })
            .with_size(Size { length: 1.0, width: 1.0, height: 1.0 })
            .build_entity(&mut world);

        assert_eq!("Entity(0, Generation(1))", format!("{:?}", cube));
    }
}