use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

use crate::components::{Speed, Target, Unit};

use amethyst::shrev::EventChannel;

use amethyst::core::nalgebra::*;

use amethyst::ecs::prelude::*;
use amethyst::prelude::*;

pub struct UnitTargetSystem;

impl<'s> System<'s> for UnitTargetSystem {
    type SystemData = (
        ReadStorage<'s, Unit>,
        WriteStorage<'s, Target>,
        Read<'s, InputHandler<String, String>>, 
    );

    fn run(&mut self, (units, mut targets, input): Self::SystemData) {
        let move_x = input.axis_value("move_x").unwrap();
        let move_y = input.axis_value("move_y").unwrap();

        for (unit, target) in (&units, &mut targets).join() {
            target.0 = target.0 + move_x as f32;
            target.1 = target.1 + move_y as f32;
        } 
    }
}

pub struct UnitMoveSystem;

impl<'s> System<'s> for UnitMoveSystem {
    type SystemData = (
        ReadStorage<'s, Target>,
        ReadStorage<'s, Speed>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (targets, speeds, mut transforms): Self::SystemData) {
        for (target, speed, transform) in (&targets, &speeds, &mut transforms).join() { // Add pathfinding later on
            let position = transform.isometry().translation.vector.clone();

            let delta_x = target.0 - position.x;
            let delta_y = target.1 - position.y;
            let delta_z = target.2 - position.z;

            let distance = (delta_x.powi(2) + delta_y.powi(2)).sqrt();

            //let angle = (delta_y/delta_x).atan(); //Or just use ratio hyp1/hyp2 = side1/side2 of the two triangles

            //println!("ANGLE: {}", angle);

            if distance > speed.0 {
                transform.translate_x(speed.0 * delta_x/distance);
                transform.translate_y(speed.0 * delta_y/distance);
            }

            //transform.face_towards(Vector3::new(target.0, target.1, target.2), Vector3::new(0.0, 0.0, 1.0));
        }
    }
}

pub struct TargetSystem {
    pub reader: Option<ReaderId<StateEvent>>,
}

impl<'s> System<'s> for TargetSystem {
    type SystemData = (
        WriteStorage<'s, Target>,
        Read<'s, EventChannel<StateEvent>>,
    );

    fn run(&mut self, (mut targets, event_channel): Self::SystemData) {
        let mut event: Option<StateEvent> = None;

        let mut iter = event_channel.read(self.reader.as_mut().unwrap());
        
        if iter.is_empty() {
            return; 
        }

        for e in iter {
            //println!("Received an event: {:?}", e);
            event = Some(e.clone()); 
        }

        for target in (&mut targets).join() {
        }
    }

    fn setup(&mut self, res: &mut amethyst::ecs::Resources) {
        Self::SystemData::setup(res);
        self.reader = Some(res.fetch_mut::<EventChannel<StateEvent>>().register_reader());
    }
}
