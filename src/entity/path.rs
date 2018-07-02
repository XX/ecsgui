use specs::{Entity, Builder, World};
use component::{Stroke, Fill, Shape};

#[derive(Debug, Default, Clone)]
pub struct Path {
    shape: Shape,
    stroke: Option<Stroke>,
    fill: Option<Fill>,
}

impl Path {
    pub fn with_shape(mut self, shape: Shape) -> Self {
        self.shape = shape;
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
        let Path { shape, stroke, fill } = self;
        let mut entity = world.create_entity()
            .with(shape);

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
    use component::ShapeCommand::*;

    #[test]
    fn build_rectangle() {
        let mut world = ::create_world();
        let path = Path::default()
            .with_shape(Shape(vec![Move((10.0, 10.0).into()), LineAlonX(90.0), LineAlonY(90.0), LineAlonX(10.0), Close]))
            .build_entity(&mut world);

        assert_eq!("Entity(0, Generation(1))", format!("{:?}", path));
    }
}