use std::panic::AssertUnwindSafe;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::time::Time;
use rand::distributions::{Distribution, Uniform};

fn random_color() -> Color {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(0..255);

    let r = die.sample(&mut rng);
    let g = die.sample(&mut rng);
    let b = die.sample(&mut rng);

    Color::rgb_u8(r, g, b)
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, do_update)
        .insert_resource(FixedTime::new_from_secs(1.0))
        .run();
}

fn do_update(mut cells: Query<(&Cell, &mut Handle<ColorMaterial>)>, mut materials: ResMut<Assets<ColorMaterial>>) {

   for(&cell, mut material) in &mut cells {
       println!("update cell {}", cell.id);
       println!("we got {} materials now loser", materials.len());
        *material =  materials.add(ColorMaterial::from(random_color()));
   }
}

#[derive(Component, Default, Copy, Clone)]
struct Cell{
     id: i32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    //setup camera
    commands.spawn(Camera2dBundle::default());

    //make mesh
    let square = meshes.add(
        shape::Quad {
            size: Vec2::new(50., 50.),
            flip: false,
        }
        .into(),
    );

    for i in 0..5 {
        //make objects
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: square.clone().into(),
                material: materials.add(ColorMaterial::from(random_color())),
                transform: Transform::from_translation(Vec3::new(50. * i as f32, 0., 0.)),
                ..default()
            },
            Cell{id: i},
        ));
    }
}
