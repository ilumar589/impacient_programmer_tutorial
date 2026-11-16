use bevy::prelude::*;

//Atlas constants
const TILE_SIZE: u32 = 64; // 64x64 tiles
const WALK_FRAMES: usize = 9; // 9 colums per walking row
const MOVE_SPEED: f32 = 140.0; // pixels per second
const ANIM_DT: f32 = 0.1; // seconds per frame (~10 FPS)

#[derive(Component)]
struct Player;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
enum Facing {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct AnimationState {
    facing: Facing,
    moving: bool,
    was_moving: bool,
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
) {
    // load the spritesheet and build a grid layout 64x64 tiles, 9 columns, 12 rows
    let whole_spritesheet_index = asset_server.load::<Image>("male_spritesheet.png");
    let layout = atlas_layout.add(TextureAtlasLayout::from_grid(
        UVec2::splat(TILE_SIZE),
        WALK_FRAMES as u32, //columns used for walking frames
        12,                 // at least 12 rows available
        None,
        None,
    ));

    // Start facing down (towards user), idle on first frame of that row
    let facing = Facing::Down;
    let start_index = atlas_index_for(facing, 0);

    commands.spawn((
        Sprite::from_atlas_image(
            whole_spritesheet_index,
            TextureAtlas {
                layout,
                index: start_index,
            },
        ),
        Transform::from_translation(Vec3::ZERO),
        Player,
        AnimationState {
            facing,
            moving: false,
            was_moving: false,
        },
        AnimationTimer(Timer::from_seconds(ANIM_DT, TimerMode::Repeating)),
    ));
}

fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &mut AnimationState), With<Player>>,
) {
    let Ok((mut transform, mut anim)) = player_query.single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;

    if input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }

    if input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    if input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }

    if input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }

    if direction != Vec2::ZERO {
        let speed = 300.0; //pixels per second
        let delta = direction.normalize() * speed * time.delta_secs();
        transform.translation.x += delta.x;
        transform.translation.y += delta.y;

        // update facing based on dominant direction
        if direction.x.abs() > direction.y.abs() {
            anim.facing = if direction.x > 0.0 {
                Facing::Right
            } else {
                Facing::Left
            }
        } else {
            anim.facing = if direction.y > 0.0 {
                Facing::Up
            } else {
                Facing::Down
            }
        }
    } else {
        anim.moving = false;
    }
}

// returns the starting atlas index for the given facing row. Just means row * fixed number of columns we have in the spritesheet
fn row_start_index(facing: Facing) -> usize {
    row_zero_based(facing) * WALK_FRAMES
}

// returns a specific frame from the starting atlas position
fn atlas_index_for(facing: Facing, frame_in_row: usize) -> usize {
    row_start_index(facing) + frame_in_row.min(WALK_FRAMES - 1)
}

// returns the row number (count from top to bottom of the spritesheet from 0) based where the player is facing
fn row_zero_based(facing: Facing) -> usize {
    match facing {
        Facing::Up => 8,
        Facing::Left => 9,
        Facing::Down => 10,
        Facing::Right => 11,
    }
}
