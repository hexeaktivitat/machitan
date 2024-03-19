use bevy::prelude::*;

#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct Name(String);

pub fn hello_world() {
    println!("hello, world");
}

pub fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Silence Suzuka".into())));
    commands.spawn((Person, Name("Special Week".into())));
    commands.spawn((Person, Name("Tokai Teio".into())));
}

pub fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}", name.0);
    }
}

pub fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Special Week" {
            name.0 = "Nice Nature".into();
            break;
        }
    }
}
