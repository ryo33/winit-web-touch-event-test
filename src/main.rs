use bevy::{input::touch::ForceTouch, prelude::*, utils::HashMap};

pub struct TouchEntities(pub HashMap<u64, Entity>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(TouchEntities(HashMap::default()))
        .add_startup_system(setup.after("resize"))
        .add_system(touch)
        .run();
}

fn setup(mut commands: Commands, mut windows: ResMut<Windows>) {
    let js_window = web_sys::window().unwrap();
    let window = windows.get_primary_mut().unwrap();
    let width = js_window.inner_width().unwrap().as_f64().unwrap() as f32;
    let height = js_window.inner_height().unwrap().as_f64().unwrap() as f32;
    window.set_resolution(width, height);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn touch(
    mut commands: Commands,
    touches: Res<Touches>,
    mut entities: ResMut<TouchEntities>,
    windows: Res<Windows>,
    camera: Query<&Transform, With<Camera>>,
    mut cursors: Query<(&mut Transform, &mut Sprite), Without<Camera>>,
) {
    let window = windows.get_primary().unwrap();
    let camera = camera.single();

    for touch in touches.iter() {
        let pos = cursor_pos(window, &camera, touch.position());
        let force = touch
            .force()
            .map(|f| match f {
                ForceTouch::Calibrated {
                    force,
                    max_possible_force,
                    ..
                } => force / max_possible_force,
                ForceTouch::Normalized(force) => force,
            })
            .unwrap() as f32;
        if touches.just_pressed(touch.id()) {
            let entity = commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.0, 1.0, 1.0),
                        custom_size: Some(Vec2::new(100.0 * force, 100.0 * force)),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(pos),
                    ..Default::default()
                })
                .id();
            entities.0.insert(touch.id(), entity);
        } else {
            let entity = *entities.0.get(&touch.id()).unwrap();
            let (mut transform, mut sprite) = cursors.get_mut(entity).unwrap();
            transform.translation = pos;
            sprite.custom_size = Some(Vec2::new(100.0, 100.0) * force);
        }
    }

    for touch in touches.iter_just_released() {
        let entity = entities.0.remove(&touch.id()).unwrap();
        commands.entity(entity).despawn();
    }
}

fn cursor_pos(window: &Window, camera: &Transform, pos: Vec2) -> Vec3 {
    let size = Vec2::new(window.width() as f32, window.height() as f32);
    let p = pos - size / 2.0;
    let vec = camera.compute_matrix() * p.extend(0.0).extend(1.0);
    vec.truncate()
}
