#![allow(unused)] // silence unused warnings while exploring (to comment out)

use bevy::prelude::*; 

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            width: 598.0,
            height: 676.0,
            title: "Rust Invaders!".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}
