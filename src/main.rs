mod board;
mod tetromino;

use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_startup_system(init)
        .add_plugin(board::BoardPlugin)
        .add_plugin(tetromino::TetromiroPlugin)
        .run()
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
