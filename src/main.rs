use bevy::prelude::*;
use rand::prelude::*;

const BACKGROUNDCOLOR: Color = Color::rgb(0., 0., 0.);
fn main() {
    let window = Window {
        title: "Blame Game".into(),
        resolution: (800.0, 600.0).into(),
        decorations: true,
        resizable: false,
        ..default()
    };

    App::new()
        .insert_resource(ClearColor(BACKGROUNDCOLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(window),
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(sprite_movement)
        .add_system(bevy::window::close_on_esc)
        .run();
    }

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(64.,64.)),
            ..default()
            },
        texture: asset_server.load("wtf-logo.png"),
        transform: Transform::from_xyz(-200.,200.,1.),
            ..default()
    },
//    Direction::Up,
    ));
}

#[derive(Component)]
struct Ball {
    direction: Vec2,
}

fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Transform)>){
    let offset = 32.;
    let mut dx = 150.;
    let mut dy = 150.;
    for mut transform in &mut sprite_position {
        if transform.translation.y < 300. - offset {
            transform.translation.y += dy * time.delta_seconds();
        } else if transform.translation.y > -300. + offset{
            transform.translation.y -= dy * time.delta_seconds();
        }
        if transform.translation.x < 400. - offset {
            transform.translation.x += dx * time.delta_seconds();
        } else if transform.translation.x > -400. + offset
        { transform.translation.x -= dx * time.delta_seconds();
        }
    }
}
