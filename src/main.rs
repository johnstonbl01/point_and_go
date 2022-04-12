use bevy::prelude::*;

#[derive(Component)]
struct Rectangle {
    speed: f32,
}

#[derive(Component)]
struct MainCamera;

struct MouseClickLoc {
    point: Vec2,
}

fn main() {
    App::new()
        .insert_resource(MouseClickLoc {
            point: Vec2::new(0.0, 0.0),
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(mouse_click)
        .add_system(move_rectangle)
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.25),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Rectangle { speed: 5.0 });
}

fn mouse_click(
    mut mouse_loc: ResMut<MouseClickLoc>,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    query_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = query_camera.single();
    if mouse_button_input.just_pressed(MouseButton::Right) {
        let world_coords = get_world_cursor_position(windows, camera, camera_transform);

        match world_coords {
            Some(coords) => mouse_loc.point = coords,
            None => info!("Could not determine mouse coords"),
        }
    }
}

fn get_world_cursor_position(
    windows: Res<Windows>,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec2> {
    let win = windows.get(camera.window).unwrap();

    if let Some(screen_pos) = win.cursor_position() {
        let window_size = Vec2::new(win.width() as f32, win.height() as f32);

        let gpu_coords = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let gpu_to_world_matrix =
            camera_transform.compute_matrix() * camera.projection_matrix.inverse();

        let world_coords: Vec2 = gpu_to_world_matrix
            .project_point3(gpu_coords.extend(-1.0))
            .truncate();

        Some(world_coords)
    } else {
        None
    }
}

fn calc_next_pos(click_loc: Vec2, current_loc: Vec3, speed: f32) -> Vec2 {
    let origin_vec = current_loc.truncate();

    let normalized_dif = (click_loc - origin_vec).normalize();
    let point_add = speed * normalized_dif;

    let next_pos = origin_vec + point_add;

    let mut x_val = next_pos.x;
    let mut y_val = next_pos.y;

    // A poor man's way of seeing if something is within a specific
    // radius from the coord. Likely a much better way to calculate
    // this in the future
    if next_pos.x < current_loc.x && next_pos.x < click_loc.x {
        x_val = click_loc.x;
    }

    if next_pos.x > current_loc.x && next_pos.x > click_loc.x {
        x_val = click_loc.x;
    }

    if next_pos.y < current_loc.y && next_pos.y < click_loc.y {
        y_val = click_loc.y;
    }

    if next_pos.y > current_loc.y && next_pos.y > click_loc.y {
        y_val = click_loc.y;
    }

    Vec2::new(x_val, y_val)
}

fn move_rectangle(mouse_loc: Res<MouseClickLoc>, mut query: Query<(&Rectangle, &mut Transform)>) {
    let (rectangle, mut transform) = query.single_mut();

    if transform.translation.x != mouse_loc.point.x || transform.translation.y != mouse_loc.point.y
    {
        let next_pos = calc_next_pos(mouse_loc.point, transform.translation, rectangle.speed);
        info!(
            "curr {:?} next {:?} click {:?}",
            transform.translation, next_pos, mouse_loc.point
        );
        transform.translation = next_pos.extend(transform.translation.z);
    }
}
