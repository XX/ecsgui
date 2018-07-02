use specs::{Entity, Builder, World};
use component::{Position, Stroke, Fill, Radius, Depth};
use types::Real;

#[derive(Debug, Default, Clone)]
pub struct Cylinder {
    center: Position,
    radius: Radius,
    depth: Depth,
    stroke: Option<Stroke>,
    fill: Option<Fill>,
}

impl Cylinder {
    pub fn with_center(mut self, center: Position) -> Self {
        self.center = center;
        self
    }

    pub fn with_radius(mut self, radius: Real) -> Self {
        self.radius = Radius(radius);
        self
    }

    pub fn with_depth(mut self, depth: Real) -> Self {
        self.depth = Depth(depth);
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
        let Cylinder { center, radius, depth, stroke, fill } = self;
        let mut entity = world.create_entity()
            .with(center)
            .with(radius)
            .with(depth);

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
    fn build_circle() {
        let mut world = ::create_world();
        let circle = Cylinder::default()
            .with_center(Position { x: 1.0, y: 1.0, z: 0.0})
            .with_radius(1.0)
            .build_entity(&mut world);

        assert_eq!("Entity(0, Generation(1))", format!("{:?}", circle));
    }
}