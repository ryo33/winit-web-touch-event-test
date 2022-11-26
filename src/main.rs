use bevy::{input::touch::ForceTouch, prelude::*, utils::HashMap};

#[derive(Resource)]
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

    commands.spawn(Camera2dBundle::default());
}

fn touch(
    mut commands: Commands,
    touches: Res<Touches>,
    mut entities: ResMut<TouchEntities>,
    mut cursors: Query<&mut Style>,
) {
    for touch in touches.iter() {
        let pos = touch.position();
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
                .spawn(NodeBundle {
                    style: style_from(pos, force),
                    background_color: Color::rgb(0.0, 0.0, 1.0).into(),
                    ..Default::default()
                })
                .id();
            entities.0.insert(touch.id(), entity);
        } else {
            let entity = *entities.0.get(&touch.id()).unwrap();
            let mut style = cursors.get_mut(entity).unwrap();
            *style = style_from(pos, force);
        }
    }

    for touch in touches.iter_just_released() {
        if let Some(entity) = entities.0.remove(&touch.id()) {
            commands.entity(entity).despawn();
        }
    }
    for touch in touches.iter_just_cancelled() {
        if let Some(entity) = entities.0.remove(&touch.id()) {
            commands.entity(entity).despawn();
        }
    }
}

fn style_from(pos: Vec2, force: f32) -> Style {
    let size = 100.0 * force + 100.0;
    Style {
        position_type: PositionType::Absolute,
        position: UiRect {
            left: Val::Px(pos.x - size / 2.0),
            top: Val::Px(pos.y - size / 2.0),
            ..Default::default()
        },
        size: Size::new(Val::Px(size), Val::Px(size)),
        ..Default::default()
    }
}
