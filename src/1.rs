use bevy::prelude::*;
#[derive(Component)]
struct X(pub f32);
#[derive(Component)]
struct Speed(pub f32);

fn _main1() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(_setupold)
        .run();
}

fn _setupold(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(250.9, 0.8, 1.1),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(250.9, 0.8, 1.2),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(X(0.))
        .insert(Speed(9.));
}

fn update_x(time: Res<Time>, mut query: Query<(&mut X, &mut Speed)>) {
    let dt = time.delta_seconds();
    for (mut x, speed) in query.iter_mut() {
        x.0 = dt * speed.0;
    }
}
fn moveable(mut query: Query<(&mut Transform, &X)>) {
    for (mut transform, x) in query.iter_mut() {
        transform.translation = transform.translation + Vec3::new(x.0, 0.7, 0.)
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(update_x)
        .add_system(moveable)
        .run();
}
