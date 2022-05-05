use bevy::prelude::*;

#[derive(Component)]
struct Person;
#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Name(String);


#[derive(Component)]

struct Entity(u64);

fn print_position_system(query: Query<&Transform>) {
    for transform in query.iter() {
        println!("Position:{:?}", transform.translation);
    }
}

fn hello_world() {
    println!("hello_world");
}
fn main() {
    App::new().add_system(hello_world).run();
}
