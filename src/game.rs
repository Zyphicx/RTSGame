pub struct Game;

use components::*;

use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::prelude::*;
use amethyst::renderer::{Camera, PngFormat, Projection, SpriteRender, SpriteSheet, SpriteSheetHandle, SpriteSheetFormat, Texture, TextureMetadata};

use amethyst::ecs::{Component, DenseVecStorage};

use systems::units::*;
use amethyst::winit::*;
use amethyst::shrev::{Event, EventChannel};

pub const SCREEN_WIDTH: f32 = 800.0;
pub const SCREEN_HEIGHT: f32 = 450.0;

struct Map { //Make this a structure into which a map is loaded, storing all important information
    core1_pos: Transform,
    core2_pos: Transform,
}

struct Core {
    health: f32,
}

impl Component for Core {
    type Storage = DenseVecStorage<Self>;
}

impl Core {
    fn new() -> Self {
        Core {
            health: 100.0,
        } 
    }
}

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(world);

        world.register::<Core>();

        initialise_camera(world);

       
       let mut transform = Transform::default();
       transform.set_x(12.5);
       transform.set_y(12.5);

        let map = Map {
            core1_pos: transform.clone(),
            core2_pos: transform.clone(),
        };

        println!("{:?}", map.core1_pos);

        initialise_cores(world, map, sprite_sheet_handle.clone());

        initialise_units(world, sprite_sheet_handle.clone());

        let event_channel = EventChannel::<crate::game::StateEvent>::new();
        world.add_resource(event_channel); 
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        let mut event_channel = data.world.write_resource::<EventChannel<StateEvent>>();

        event_channel.single_write(event);
        
        Trans::None
    }
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default(); //Where the camera is located so we can move it around
    transform.set_z(1.0);
    
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic( // Default camera over 2D arena
            0.0,
            SCREEN_WIDTH,
            0.0,
            SCREEN_HEIGHT,
        )))
        .with(transform)
        .build();
}


fn initialise_cores(world: &mut World, map: Map, sprite_sheet: SpriteSheetHandle) {
    let mut core1_transform = map.core1_pos;
    let mut core2_transform = map.core2_pos;

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 1,
    };

    world
        .create_entity()
        .with(Core::new())
        .with(core1_transform)
        .with(sprite_render.clone())
        .build();

    world
        .create_entity()
        .with(Core::new())
        .with(core2_transform)
        .with(sprite_render.clone())
        .build();

}

fn initialise_units(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 2,
    };

    let mut transform = Transform::default(); 

    transform.set_x(SCREEN_WIDTH/2 as f32);
    transform.set_y(SCREEN_HEIGHT/2 as f32);
    transform.set_z(0.0);

    world
        .create_entity()
        .with(transform)
        .with(Unit{unit_type: UnitType::Battler})
        .with(Speed(1.0))
        .with(Target(SCREEN_WIDTH/2 as f32, SCREEN_HEIGHT/2 as f32, 0.0))
        .with(sprite_render.clone())
        .build();
}

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>(); 
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/spritesheet.png",
            PngFormat, TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "resources/spritesheet.ron",
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store,
    )
}
