pub mod main_char{
    use::bevy::prelude::*;

    pub struct MainCharPlugin;

    impl Plugin for MainCharPlugin
    {
        fn build(&self, app: &mut App)
        {
            app
            .add_startup_system(add_sprite)
            .add_system(move_main_char);
        }
    }

    #[derive(Component)]
    struct MainChar;

    fn add_sprite(mut commands: Commands, res: Res<AssetServer>)
    {
        commands.spawn(Camera2dBundle::default());
        commands.spawn((SpriteBundle
        {
            texture: res.load("boy1/Idle (1).png"),
            transform: Transform::from_scale(Vec3::splat(0.3)),
            ..Default::default()
        }, 
        MainChar));
    }

    const SPEED: f32 = 500.0;
    fn move_main_char(keys: Res<Input<KeyCode>>, time: Res<Time>, mut char: Query<(&MainChar, &mut Transform)>)
    {
        let (_, mut transform) = char.single_mut();
        let mut direction:Vec3 = Vec3::splat(0.0);

        if keys.pressed(KeyCode::A)
        {
            direction.x = -1.0;
        }
        if keys.pressed(KeyCode::D)
        {
            direction.x = 1.0;
        }

        transform.translation += direction * time.delta_seconds() * SPEED;
    }
}