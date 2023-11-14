use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use rand::distributions::{Distribution, Uniform};

use bevy::input::mouse::MouseButtonInput;
use bevy::window::PrimaryWindow;
use bevy_editor_pls::prelude::*;

const CELL_WIDTH: f32 = 50.;
const CELL_GUTTER: f32 = 1.5;
const CELL_GRID_SIZE: i32 = 2;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EditorPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
      //  .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (bevy::window::close_on_esc, handle_mouse_click, render_world))
        .run();
}

#[derive(Component, Default, Copy, Clone)]
struct Cell {
    id: i32,
}

#[derive(Component, Default)]
struct Position {
    pos: Vec2,
}
/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
struct MyWorldCoords(Vec2);

/// Used to help identify our main camera
#[derive(Component)]
struct MainCamera;


fn render_world(cells: Query<(&Cell, &Position)>, mouse_loc: Res<MyWorldCoords>) {
    for (cell, pos) in cells.iter() {
        if mouse_loc.0.x >= pos.pos.x && mouse_loc.0.x < (pos.pos.x + CELL_WIDTH) {
            println!("we are over cell id {}", cell.id);
        }
    }

}
fn handle_mouse_click(
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut mycoords: ResMut<MyWorldCoords>, mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    use bevy::input::ButtonState;

    for ev in mousebtn_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                println!("Mouse button press: {:?}", ev.button);
                if let Some(position) = q_windows.single().cursor_position() {
                    println!("Cursor is inside the primary window, at {:?}", position);
                }
                // get the camera info and transform
                // assuming there is exactly one main camera entity, so Query::single() is OK
                let (camera, camera_transform) = q_camera.single();

                // There is only one primary window, so we can similarly get it from the query:
                let window = q_windows.single();

                // check if the cursor is inside the window and get its position
                // then, ask bevy to convert into world coordinates, and truncate to discard Z
                if let Some(world_position) = window.cursor_position()
                    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                    .map(|ray| ray.origin.truncate())
                {
                    mycoords.0 = world_position;
                    eprintln!("World coords: {}/{}", world_position.x, world_position.y);

                    //make mesh
                    let square = meshes.add(
                        shape::Quad {
                            size: Vec2::new(CELL_WIDTH, CELL_WIDTH),
                            flip: false,
                        }
                            .into(),
                    );
                    let black_handle = materials.add(ColorMaterial::from(Color::BLACK));

                    commands.spawn((
                        MaterialMesh2dBundle {
                            mesh: square.clone().into(),
                            material: black_handle.clone(),
                            transform: Transform::from_translation(mycoords.0.extend(0.0)),
                            ..default()
                        },
                        Cell { id: rand::random() },
                        Position{ pos: mycoords.0 }
                    ));

                }
            }
            ButtonState::Released => {
                println!("Mouse button release: {:?}", ev.button);
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    //setup camera
    commands.init_resource::<MyWorldCoords>();
    // Make sure to add the marker component when you set up your camera
    commands.spawn((Camera2dBundle::default(), MainCamera));

    //make mesh
    let square = meshes.add(
        shape::Quad {
            size: Vec2::new(CELL_WIDTH, CELL_WIDTH),
            flip: false,
        }
        .into(),
    );

    let color_handles = vec![
        materials.add(ColorMaterial::from(Color::RED)),
        materials.add(ColorMaterial::from(Color::BLUE)),
        materials.add(ColorMaterial::from(Color::GREEN)),
        materials.add(ColorMaterial::from(Color::YELLOW)),
        materials.add(ColorMaterial::from(Color::ORANGE)),
    ];

    let black_handle = materials.add(ColorMaterial::from(Color::BLACK));

    println!("making objects");
    let gutter_size = (CELL_WIDTH * CELL_GUTTER) - CELL_WIDTH;
    let offset = (CELL_WIDTH / 2.) + gutter_size;



    //make objects
    for i in 0..CELL_GRID_SIZE {
        for j in 0..CELL_GRID_SIZE {
            let c = if i != j {
                black_handle.clone()
            } else {
                color_handles[i as usize].clone()
            };

            let x = ((CELL_WIDTH * i as f32) * CELL_GUTTER) - offset;
            let y = ((CELL_WIDTH * j as f32) * CELL_GUTTER) - offset;
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: square.clone().into(),
                    material: c,
                    transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                    ..default()
                },
                Cell { id: i },
                Position {pos: Vec2{ x, y }}
            ));
        }
    }
}
