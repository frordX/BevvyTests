use::bevy::prelude::*;
use crate::main_char::main_char::MainCharPlugin;
mod main_char;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(MainCharPlugin)
    .run();
}