use crate::game::{Wall, Floor, spawn_trash, update_on_wrong_letter};
use crate::game_over::{is_game_over, GameOverLine, GameOver};
use crate::loading::TextureAssets;
use crate::GameState;
use crate::score::{Score, ComboModifier};
use crate::trash_text::{TrashText, TrashTextBundle, highlight_characters, remove_highlight};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::transform::TransformSystem;
use bevy::utils::HashMap;
use bevy_rapier2d::prelude::*;
use crate::typing::{typing, TypingBuffer};

pub const WINDOW_WIDTH: f32 = 700.0;
pub const WINDOW_HEIGHT: f32 = 800.0;

const BORDER_TILE_SIZE: f32 = 48.0;
const BORDER_TILE_SCALE: Vec2 = Vec2::new(BORDER_TILE_SIZE, BORDER_TILE_SIZE);

const TRASH_STARTING_VELOCITY: Vec2 = Vec2::new(0.0, -100.0);
const TRASH_MAXIMUM_VERTICAL_VELOCITY_LENGTH: f32 = 40.0;
const TRASH_MAXIMUM_HORIZONTAL_VELOCITY_LENGTH: f32 = 600.0;
const TRASH_SPAWN_DISTANCE_BETWEEN_SPAWNS: f32 = 30.0;

const INITIAL_TRASH_SPAWN_RATE: f32 = 2.0;
const INITIAL_DIFICULTY_INCREASE_RATE: f32 = 10.0;

pub struct TrashPlugin;


#[derive(Clone, Debug)]
pub enum RunState {
    Running,
    Ended,
}


#[derive(Clone, Debug)]
pub enum TrashType {
    Bottle,
    Pizza,
    BigBox,
    GlassBottle,
    News,
    Shampoo,
    SmallCan,
    Soda,
    Spray,
}

impl Default for TrashType {
    fn default() -> Self {
        Self::Bottle
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PowerUp {
    None,
    Explosion,
    Link
}

impl Default for PowerUp {
    fn default() -> Self {
        Self::None
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum PowerUpEvent {
    None,
    EntityDestroyed,
    DestroyLinked,
}


#[derive(Component, Debug, Clone)]
pub struct Trash {
    pub trash_type: TrashType,
    pub power_up: PowerUp,
    pub size: Vec2,
    pub activated: bool,
}

impl Default for Trash {
    fn default() -> Self {
        Self::bottle()
    }
}


#[derive(Component, Debug, Clone, Default)]
pub struct BufferText;


#[derive(Component, Default)]
pub struct TrashActionActive;

#[derive(Component, Default)]
pub struct TrashActionDuplicate;

#[derive(Component, Default)]
pub struct TrashMarked;

#[derive(Component, Default)]
pub struct TrashExplosion;


// #[derive(Resource)]
// pub struct TrashSpawnTimer(pub Timer);
//
// #[derive(Resource)]
// struct DifficultyTimer(pub Timer);

#[derive(Resource)]
struct BufferTextDeleteTimer(Timer);

// #[derive(Resource)]
// pub struct TypingBuffer(String);

#[derive(Resource)]
pub struct AvailableWords(HashMap<String, Vec<String>>);


#[derive(Bundle)]
pub struct TrashBundle {
    sprite: SpriteBundle,
    rigidbody: RigidBody,
    velocity: Velocity,
    collider: Collider,
    collider_mass_properties: ColliderMassProperties,
    restitution: Restitution,
    active_events: ActiveEvents,
    trash: Trash,
}

impl TrashBundle {
    pub fn new(sprite: Handle<Image>, trash: Trash) -> Self {
        Self {
            sprite: SpriteBundle {
                texture: sprite,
                ..default()
            },
            rigidbody: RigidBody::Dynamic,
            velocity: Velocity::linear(TRASH_STARTING_VELOCITY),
            collider: Collider::cuboid(trash.size.x, trash.size.y),
            collider_mass_properties: ColliderMassProperties::Mass(1.0),
            restitution: Restitution::coefficient(0.2),
            active_events: ActiveEvents::COLLISION_EVENTS,
            trash: trash.clone(),
        }
    }

    pub fn create_text(text: String, anchor: Anchor, color: Color, style: TextStyle) -> TrashTextBundle {
        TrashTextBundle::new(
            text,
            anchor,
            color,
            style,
        )
    }
}


/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for TrashPlugin {
    fn build(&self, app: &mut App) {
        // app.insert_resource(TrashSpawnTimer(Timer::from_seconds(INITIAL_TRASH_SPAWN_RATE, TimerMode::Repeating)))
        // .insert_resource(DifficultyTimer(Timer::from_seconds(INITIAL_DIFICULTY_INCREASE_RATE, TimerMode::Repeating)))
        // .insert_resource(TypingBuffer("".to_string()))
        // .insert_resource(AvailableWords(get_available_words_from_file()))
        app.add_systems(OnEnter(GameState::Playing), setup)
        .add_systems(Update, (
                // spawn_trash.run_if(in_state(GameState::Playing)),
                trash_power_ups_effects.after(spawn_trash),
                // update_difficuly.after(setup).run_if(in_state(GameState::Playing)),
                // typing.after(setup).run_if(in_state(GameState::Playing)),
                activate_matching_trash.after(update_on_wrong_letter).before(handle_trash_collision),
                highlight_character.after(update_on_wrong_letter),
                handle_trash_collision.after(activate_matching_trash).run_if(in_state(GameState::Playing)),
                clamp_duplicated_trash.after(handle_trash_collision),
                remove_explosions.after(handle_trash_collision),
                // update_buffer_text.after(typing),
                // click_restart_button.run_if(in_state(GameState::GameOver)),
            )
        )
        .add_systems(PostStartup, fix_trash_label_rotation.before(TransformSystem::TransformPropagate).run_if(in_state(GameState::Playing)))
        .add_systems(PostUpdate, fix_trash_label_rotation.before(TransformSystem::TransformPropagate).run_if(in_state(GameState::Playing)));
        // .add_systems(OnEnter(GameState::GameOver), spawn_game_over_menu)
        // .add_systems(OnExit(GameState::Playing), delete_all_play_entities);
        // .add_systems(OnExit(GameState::GameOver), delete_all_gameover_entities);
    }
}

fn setup() {

}

impl Trash {
    pub fn get_by_type(trash_type: TrashType) -> Self {
        match trash_type {
            TrashType::Bottle => Self::bottle(),
            TrashType::Pizza => Self::pizza(),
            TrashType::News => Self::news(),
            TrashType::Shampoo => Self::shampoo(),
            TrashType::SmallCan => Self::small_can(),
            TrashType::Soda => Self::soda(),
            TrashType::Spray => Self::spray(),
            TrashType::BigBox => Self::big_box(),
            TrashType::GlassBottle => Self::glass_bottle(),
        }
    }

    pub fn bottle() -> Self {
        Self {
            trash_type: TrashType::Bottle,
            size: Vec2::new(15.0, 16.0),
            power_up: PowerUp::None,
            activated: false,
        }
    }

    pub fn pizza() -> Self {
        Self {
            trash_type: TrashType::Pizza,
            size: Vec2::new(32.0, 16.0), // 60x30
            power_up: PowerUp::None,
            activated: false,
        }
    }

    pub fn big_box() -> Self {
        Self {
            trash_type: TrashType::BigBox,
            size: Vec2::new(25.0, 24.0), // 51x48
            power_up: PowerUp::None,
            activated: false,
        }
    }

    pub fn glass_bottle() -> Self {
        Self {
            trash_type: TrashType::GlassBottle,
            size: Vec2::new(8.0, 25.0), // 17x50
            power_up: PowerUp::None,
            activated: false,
        }
    }

    pub fn news() -> Self {
        Self {
            trash_type: TrashType::News,
            size: Vec2::new(26.0, 16.0), // 53x28
            power_up: PowerUp::None,
            activated: false,
        }
    }

    pub fn shampoo() -> Self {
        Self {
            trash_type: TrashType::Shampoo,
            size: Vec2::new(17.0, 22.0), // 36x44
            power_up: PowerUp::None,
            activated: false,
        }
    }

    pub fn small_can() -> Self {
        Self {
            trash_type: TrashType::SmallCan,
            size: Vec2::new(11.0, 15.0), // 23x30
            power_up: PowerUp::None,
            activated: false,
        }
    }

    pub fn soda() -> Self {
        Self {
            trash_type: TrashType::Soda,
            size: Vec2::new(9.0, 17.0), // 18x34
            power_up: PowerUp::None,
            activated: false,
        }
    }

    pub fn spray() -> Self {
        Self {
            trash_type: TrashType::Spray,
            size: Vec2::new(8.0, 20.0), // 17x40
            power_up: PowerUp::None,
            activated: false,
        }
    }
}


pub fn get_trash_sprite(trash_type: &TrashType, textures: &Res<TextureAssets>) -> Handle<Image> {
    match trash_type {
        TrashType::Bottle => textures.bottle.clone(),
        TrashType::Pizza => textures.pizza.clone(),
        TrashType::BigBox => textures.big_box.clone(),
        TrashType::GlassBottle => textures.glass_bottle.clone(),
        TrashType::News => textures.news.clone(),
        TrashType::Shampoo => textures.shampoo.clone(),
        TrashType::SmallCan => textures.small_can.clone(),
        TrashType::Soda => textures.soda.clone(),
        TrashType::Spray => textures.spray.clone(),
    }
}



// https://github.com/bevyengine/bevy/issues/1780#issuecomment-1760929069
fn fix_trash_label_rotation(
    mut text_query: Query<(&Parent, &mut Transform), With<TrashText>>,
    query_parents: Query<&Transform, (With<Trash>, Without<TrashText>, With<TrashActionActive>)>,
) {
    for (parent, mut transform) in text_query.iter_mut() {
        if let Ok(parent_transform) = query_parents.get(parent.get()) {
            transform.rotation = parent_transform.rotation.inverse();
        }
    }
}


fn remove_all_marked_trash(
    mut commands: Commands,
    marked_trash_query: Query<Entity, With<TrashMarked>>,
) {
    for entity in &mut marked_trash_query.iter() {
        commands.entity(entity).remove::<TrashMarked>();
    }
}


fn trash_power_ups_effects(
    mut active_trash_query: Query<(&Trash, &mut Sprite), With<TrashActionActive>>,
    mut inactive_trash_query: Query<&mut Sprite, Without<TrashActionActive>>,
    time: Res<Time>,
) {
    let seconds = time.elapsed_seconds();
    let color_change_interval = (3.5 * seconds).sin() / 2.0 + 0.5;

    for (trash, mut sprite) in &mut active_trash_query.iter_mut() {
        match trash.power_up {
            PowerUp::Explosion => {
                sprite.color = Color::rgb(
                    1.0,
                    color_change_interval,
                    color_change_interval
                );
            },
            PowerUp::Link => {
                sprite.color = Color::rgb(
                    color_change_interval,
                    color_change_interval,
                    1.0,
                );
            },
            _ => {}
        }
    }

    for mut sprite in &mut inactive_trash_query.iter_mut() {
        if sprite.color != Color::WHITE {
            sprite.color = Color::WHITE;
        }
    }
}


fn highlight_character(
    mut trash_query: Query<(&TrashText, &mut Text)>,
    typing_buffer: Res<TypingBuffer>,
) {
    for (trash_text, mut ui_text) in &mut trash_query.iter_mut() {
        if trash_text.word.starts_with(&typing_buffer.0) {
            ui_text.sections = highlight_characters(&ui_text.sections, typing_buffer.0.len(), trash_text.highlight_color)
        } else {
            ui_text.sections = remove_highlight(&ui_text.sections, trash_text.color)
        }
    }
}

fn remove_explosions(
    mut commands: Commands,
    explosion_query: Query<Entity, With<TrashExplosion>>,
) {
    for entity in &mut explosion_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}


fn remove_trash_text(commands: &mut Commands, trash_entity: &Entity) {
    commands.entity(*trash_entity).remove::<TrashActionActive>();
    commands.entity(*trash_entity).remove::<TrashMarked>();
    commands.entity(*trash_entity).despawn_descendants();

}

fn create_duplicated_trash_from_entity(commands: &mut Commands, sprite: Handle<Image>, trash: Trash, transform: Transform) {
    let mut trash = trash.clone();
    trash.power_up = PowerUp::None;
    trash.activated = false;

    let mut bundle = TrashBundle::new(sprite, trash);
    bundle.velocity = Velocity::zero();

    commands.spawn(bundle)
        .insert(transform)
        .insert(GravityScale(40.0));
}

fn should_delete_text(
    entity: &Entity,
    other: &Entity,
    active_trash_query: &Query<(Entity, &Velocity, &Trash, &Transform), With<TrashActionActive>>,
    inactive_trash_query: &Query<(Entity, &Velocity), (Without<TrashActionActive>, With<Trash>)>,
    walls_query: &Query<Entity, With<Wall>>,
    floor_query: &Query<Entity, With<Floor>>,

) -> bool {

    if walls_query.get(*entity).is_ok() || walls_query.get(*other).is_ok() {
        return false;
    }

    if let Ok(_) = active_trash_query.get(*entity) {
        if floor_query.get(*other).is_ok() {
            return true;
        }

        if active_trash_query.get(*other).is_err() {
            if let Ok(inactive_trash) = inactive_trash_query.get(*other) {
                if inactive_trash.1.linvel.length() < TRASH_STARTING_VELOCITY.length() / 2.0 {
                    return true;
                }
            }
        }
    }

    return false;
}

fn should_duplicate_trash(
    entity: &Entity,
    other: &Entity,
    duplicate_trash_query: &Query<(Entity, &Trash, &Transform, &Handle<Image>), With<TrashActionDuplicate>>,
    walls_query: &Query<Entity, With<Wall>>,
    floor_query: &Query<Entity, With<Floor>>,
    game_over_query: &Query<Entity, With<GameOverLine>>,

) -> bool {
    if duplicate_trash_query.get(*entity).is_ok() {
        if floor_query.get(*other).is_err() &&
            walls_query.get(*other).is_err() && 
            game_over_query.get(*other).is_err() {
            return true;
        }
    }

    false
}


fn should_explode(
    entity: &Entity,
    other: &Entity,
    explosion_query: &Query<&Transform, With<TrashExplosion>>,
    trash_query: &Query<Entity, With<Trash>>
) -> bool {

    if let Ok(_) = explosion_query.get(*other) {
        if trash_query.get(*entity).is_ok() {
            return true;
        }
    }

    false
}


fn handle_power_up_event(
    entity: &Entity,
    commands: &mut Commands,
    trash_query: &Query<(Entity, &Velocity, &Trash, &Transform), With<TrashActionActive>>,
) -> PowerUpEvent {
    if let Ok((_, _, trash, transform)) = trash_query.get(*entity) {
        if trash.activated {
            match trash.power_up {
                PowerUp::Explosion => {
                    commands.spawn(
                        SpriteBundle {
                            ..default()
                        }
                    )
                    .insert(transform.clone())
                    .insert(Collider::cuboid(50.0, 50.0))
                    .insert(Sensor)
                    .insert(TrashExplosion);

                    return PowerUpEvent::EntityDestroyed;
                },
                PowerUp::Link => {
                    return PowerUpEvent::DestroyLinked;
                },
                _ => {}
            }
        }
    }

    PowerUpEvent::None
}

fn handle_trash_entity_collision(
    entity: &Entity,
    other: &Entity,
    commands: &mut Commands,
    typing_buffer: &mut ResMut<TypingBuffer>,
    active_trash_query: &Query<(Entity, &Velocity, &Trash, &Transform), With<TrashActionActive>>,
    inactive_trash_query: &Query<(Entity, &Velocity), (Without<TrashActionActive>, With<Trash>)>,
    duplicate_trash_query: &Query<(Entity, &Trash, &Transform, &Handle<Image>), With<TrashActionDuplicate>>,
    marked_trash_query: &Query<Entity, With<TrashMarked>>,
    explosion_query: &Query<&Transform, With<TrashExplosion>>,
    all_trash_query: &Query<Entity, With<Trash>>,
    walls_query: &Query<Entity, With<Wall>>,
    floor_query: &Query<Entity, With<Floor>>,
    game_over_query: &Query<Entity, With<GameOverLine>>,

) {
    let mut powerup_event = PowerUpEvent::None;
    if game_over_query.get(*entity).is_err() && game_over_query.get(*other).is_err() {
        powerup_event = handle_power_up_event(entity, commands, active_trash_query);
    }

    if should_explode(entity, other, explosion_query, all_trash_query) {
        if marked_trash_query.get(*entity).is_ok() {
            typing_buffer.0 = "".to_string();
        }
        commands.entity(*entity).despawn_recursive();
        return;
    }

    let mut should_remove_text = false;
    if should_delete_text(entity, other, active_trash_query, inactive_trash_query, walls_query, floor_query) {
        if marked_trash_query.get(*entity).is_ok() {
            typing_buffer.0 = "".to_string();
        }
        should_remove_text = true;

    }

    if should_duplicate_trash(entity, other, duplicate_trash_query, walls_query, floor_query, game_over_query) {
        let trash_data = duplicate_trash_query.get(*entity).unwrap();
        commands.entity(trash_data.0).remove::<TrashActionDuplicate>();
        let mut transform = trash_data.2.clone();
        transform.translation.y += 10.0;
        create_duplicated_trash_from_entity(
            commands,
            trash_data.3.clone(),
            trash_data.1.clone(),
            trash_data.2.clone());
    }


    if powerup_event != PowerUpEvent::EntityDestroyed && should_remove_text {
        remove_trash_text(commands, entity);
    }
}


pub fn handle_trash_collision(
    mut commands: Commands,
    mut typing_buffer: ResMut<TypingBuffer>,
    mut collision_events: EventReader<CollisionEvent>,
    mut next_state: ResMut<NextState<GameState>>,
    active_trash_query: Query<(Entity, &Velocity, &Trash, &Transform), With<TrashActionActive>>,
    inactive_trash_query: Query<(Entity, &Velocity), (Without<TrashActionActive>, With<Trash>)>,
    duplicate_trash_query: Query<(Entity, &Trash, &Transform, &Handle<Image>), With<TrashActionDuplicate>>,
    marked_trash_query: Query<Entity, With<TrashMarked>>,
    explosion_query: Query<&Transform, With<TrashExplosion>>,
    all_trash_query: Query<Entity, With<Trash>>,
    walls_query: Query<Entity, With<Wall>>,
    floor_query: Query<Entity, With<Floor>>,
    game_over_query: Query<Entity, With<GameOverLine>>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {

                if is_game_over(entity1, entity2, &inactive_trash_query, &game_over_query) {
                    commands.spawn(GameOver);
                    next_state.set(GameState::GameOver);
                    return;
                }

                handle_trash_entity_collision(
                    entity1,
                    entity2,
                    &mut commands,
                    &mut typing_buffer,
                    &active_trash_query,
                    &inactive_trash_query,
                    &duplicate_trash_query,
                    &marked_trash_query,
                    &explosion_query,
                    &all_trash_query,
                    &walls_query,
                    &floor_query,
                    &game_over_query
                );

                handle_trash_entity_collision(
                    entity2,
                    entity1,
                    &mut commands,
                    &mut typing_buffer,
                    &active_trash_query,
                    &inactive_trash_query,
                    &duplicate_trash_query,
                    &marked_trash_query,
                    &explosion_query,
                    &all_trash_query,
                    &walls_query,
                    &floor_query,
                    &game_over_query
                );
            }
            CollisionEvent::Stopped(_entity1, _entity2, _) => {},
        }
    }
}


fn clamp_duplicated_trash(
    mut trash_query: Query<&mut Velocity, (With<Trash>, Without<TrashActionActive>)>,
) {
    for mut velocity in trash_query.iter_mut() {
        if velocity.linvel.length() > TRASH_STARTING_VELOCITY.length() / 2.0 {
            velocity.linvel = velocity.linvel.clamp(Vec2::new(0.0, 0.0), Vec2::new(TRASH_MAXIMUM_HORIZONTAL_VELOCITY_LENGTH, TRASH_MAXIMUM_VERTICAL_VELOCITY_LENGTH ));
        }
        // println!("Velocity: {:?}", velocity.linvel.length());
        // if velocity.linvel.length() < TRASH_STARTING_VELOCITY.length() / 2.0 {
        //     remove_trash_text(&mut commands, &entity);
        // }
    }
}


fn activate_matching_trash(
    mut commands: Commands,
    trash_text_query: Query<(&Parent, &Transform, &TrashText)>,
    mut trash_query: Query<(&mut Trash, &Transform), With<TrashActionActive>>,
    mut typing_buffer: ResMut<TypingBuffer>,
    mut score: ResMut<Score>,
    combo_modifier: Res<ComboModifier>,
) {

    if !typing_buffer.is_changed() {
        return;
    }

    // let mut trash_to_destroy: Vec<&Parent> = Vec::new();
    let mut should_clear_buffer = false;

    for (entity, _transform, trash_text) in &mut trash_text_query.iter() {
        if typing_buffer.0 == trash_text.word {
            if let Ok(mut trash) = trash_query.get_mut(entity.get()) {
                if trash.0.activated {
                    continue;
                }

                match trash.0.power_up {
                    PowerUp::Explosion => {
                        commands.entity(entity.get()).remove::<TrashMarked>();
                        commands.entity(entity.get()).despawn_descendants();
                        // remove_trash_text(&mut commands, entity);
                        // commands.spawn(TrashExplosion)
                        //     .insert(trash.1.clone());
                    },
                    PowerUp::Link => {
                        commands.entity(entity.get()).remove::<TrashMarked>();
                        commands.entity(entity.get()).despawn_descendants();

                    },
                    PowerUp::None => {
                        commands.entity(entity.get()).despawn_recursive();
                    }
                }

                trash.0.activated = true;
                score.0 += trash_text.word.len() * combo_modifier.0;
                should_clear_buffer = true;
            }
            // trash_to_destroy.push(entity);
        }
    }
    // for entity in &trash_to_destroy {
    //     commands.entity(entity.get()).despawn_recursive();
    // }

    if should_clear_buffer {
        typing_buffer.0 = "".to_string();
    }
}
