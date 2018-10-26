use amethyst::prelude::*;
use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::Time;
use amethyst::core::cgmath::{One, Basis2, Vector2, Vector3, Matrix4};
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::core::cgmath::{Point2, Rad, Rotation2};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::renderer::{
    Camera,
    MaterialTextureSet,
    Projection,
    PngFormat,
    Texture,
    TextureMetadata,
    SpriteSheet,
    SpriteSheetFormat,
    SpriteSheetHandle,
    SpriteRender,
    VirtualKeyCode,
};
use amethyst::ecs::prelude::*;
use amethyst_rhusics::{setup_2d_arena, time_sync};
use amethyst_rhusics::rhusics_core::{CollisionMode, CollisionStrategy, Material, Pose, PhysicalEntity};
use amethyst_rhusics::rhusics_core::collide2d::{BodyPose2, Circle, Rectangle};
use amethyst_rhusics::rhusics_core::physics2d::{CollisionShape2, Mass2, Velocity2};
use amethyst_rhusics::rhusics_ecs::{DeltaTime, WithPhysics};

use systems::Gravity;

pub type BodyPose = BodyPose2<f32>;
pub type Mass = Mass2<f32>;
pub type Velocity = Velocity2<f32>;
pub type CollisionShape = CollisionShape2<f32, BodyPose>;

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

#[derive(Default)]
struct Pause {
    saved_time_scale: f32,
}

impl<'a, 'b> SimpleState<'a, 'b> for Pause {

    fn on_start(&mut self, data: StateData<GameData>) {
        let mut time = data.world.write_resource::<Time>();
        self.saved_time_scale = time.time_scale();
        time.set_time_scale(0.0);
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        let mut time = data.world.write_resource::<Time>();
        time.set_time_scale(self.saved_time_scale);
    }

    fn handle_event(&mut self, _: StateData<GameData>, event: StateEvent) -> SimpleTrans<'a, 'b> {
        if let StateEvent::Window(wevent) = event {
            if is_close_requested(&wevent) || is_key_down(&wevent, VirtualKeyCode::Escape) {
                return Trans::Quit;
            } else if is_key_down(&wevent, VirtualKeyCode::Space) {
                return Trans::Pop;
            }
        }

        Trans::None
    }
}

pub struct Pong;

impl Pong {
    fn load_sprite_sheet(resources: &Resources) -> SpriteSheetHandle {
        let texture = {
            let loader = resources.fetch::<Loader>();
            let texture_storage = resources.fetch::<AssetStorage<Texture>>();
            loader.load(
                "textures/pong_spritesheet.png",
                PngFormat,
                TextureMetadata::srgb_scale(),
                (),
                &texture_storage,
            )
        };

        let texture_id = 0;
        let mut material_texture_set = resources.fetch_mut::<MaterialTextureSet>();
        material_texture_set.insert(texture_id, texture);
        let loader = resources.fetch::<Loader>();
        let spritesheet_storage = resources.fetch::<AssetStorage<SpriteSheet>>();
        loader.load(
            "textures/pong_spritesheet.ron",
            SpriteSheetFormat,
            texture_id,
            (),
            &spritesheet_storage,
        )
    }
}

impl<'a, 'b> SimpleState<'a, 'b> for Pong {

    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        setup_2d_arena(
            Point2::new(0.0, 0.0),
            Point2::new(ARENA_WIDTH, ARENA_HEIGHT),
            ((), (), (), ()),
            world
        );

        world.add_resource(Gravity(0.0));

        let sprite_sheet = Pong::load_sprite_sheet(&world.res);
        initialise_paddles(world, sprite_sheet);
        initialise_camera(world);
    }

    fn handle_event(&mut self, _: StateData<GameData>, event: StateEvent) -> SimpleTrans<'a, 'b> {
        if let StateEvent::Window(wevent) = event {
            if is_close_requested(&wevent) || is_key_down(&wevent, VirtualKeyCode::Escape) {
                return Trans::Quit;
            } else if is_key_down(&wevent, VirtualKeyCode::Space) {
                return Trans::Push(Box::new(Pause::default()));
            }
        }

        Trans::None
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans<'a, 'b> {
        time_sync(data.world);

        Trans::None
    }

    fn on_pause(&mut self, data: StateData<GameData>) {
        // Pause the physics
        data.world.write_resource::<DeltaTime<f32>>().delta_seconds = 0.0;
    }
}

fn initialise_camera(world: &mut World) {
    world.create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            ARENA_HEIGHT,
            0.0,
        )))
        .with(GlobalTransform(
            Matrix4::from_translation(Vector3::new(0.0, 0.0, 1.0)).into()
        ))
        .build();
}

/// Initialises one paddle on the left, and one paddle on the right.
fn initialise_paddles(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let x = ARENA_WIDTH / 2.0;
    let y = ARENA_HEIGHT / 2.0;

    let paddle_sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
        flip_horizontal: false,
        flip_vertical: false,
    };

    let ball_sprite = SpriteRender {
        sprite_sheet,
        sprite_number: 1,
        flip_horizontal: false,
        flip_vertical: false,
    };

    world.register::<Ball>();
    world.register::<Paddle>();

    // Create a left plank entity.
    create_paddle(world, Side::Left, paddle_sprite.clone(), PADDLE_WIDTH * 1.5);
    create_paddle(world, Side::Right, paddle_sprite, ARENA_WIDTH - PADDLE_WIDTH * 1.5);

    world
        .create_entity()
        .with(Ball())
        .with(GlobalTransform::default())
        .with(Transform::default())
        .with_dynamic_physical_entity(
            CollisionShape::new_simple(
                CollisionStrategy::FullResolution,
                CollisionMode::Discrete,
                Circle::new(BALL_WIDTH / 2.0).into()
            ),
            BodyPose::new(Point2::new(x - BALL_WIDTH * 0.5, y - BALL_HEIGHT * 0.5), Rotation2::from_angle(Rad(0.0))),
            Velocity::from_linear(Vector2::new(-18.0, 2.0)),
            PhysicalEntity::new(Material::new(0.6, 1.0)),
            Mass::new_with_inertia(1.0f32, 1.0f32)
        )
        .with(ball_sprite)
        .build();
}

fn create_paddle(world: &mut World, side: Side, sprite: SpriteRender, x: f32) {
    world
        .create_entity()
        .with(Paddle::new(side))
        .with(GlobalTransform::default())
        .with(Transform::default())
        .with_dynamic_physical_entity(
            CollisionShape::new_simple(
                CollisionStrategy::FullResolution,
                CollisionMode::Discrete,
                Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT).into()
            ),
            BodyPose::new(Point2::new(x, ARENA_HEIGHT / 2.0), Basis2::one()),
            Velocity::default(),
            PhysicalEntity::new(Material::new(1.0, 1.0)),
            Mass::new_with_inertia(4.0f32, 20.0f32)
        )
        .with(sprite)
        .build();
}

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    fn new(side: Side) -> Paddle {
        Paddle {
            side: side,
            width: 1.0,
            height: 1.0,
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

pub const BALL_HEIGHT: f32 = 4.0;
pub const BALL_WIDTH: f32 = BALL_HEIGHT;

pub struct Ball ();

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}
