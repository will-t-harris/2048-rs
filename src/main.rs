use bevy::prelude::*;
use itertools::Itertools;
use rand::prelude::*;
use std::convert::TryFrom;

const TILE_SIZE: f32 = 40.0;
const TILE_SPACER: f32 = 10.0;

struct Board {
    size: u8,
    physical_size: f32,
}

impl Board {
    fn new(size: u8) -> Self {
        let physical_size = f32::from(size) * TILE_SIZE + f32::from(size + 1) * TILE_SPACER;

        Board {
            size,
            physical_size,
        }
    }
    fn cell_position_to_physical(&self, pos: u8) -> f32 {
        let offset = -self.physical_size / 2.0 + 0.5 * TILE_SIZE;

        offset + f32::from(pos) * TILE_SIZE + f32::from(pos + 1) * TILE_SPACER
    }
}

struct Materials {
    board: Handle<ColorMaterial>,
    tile_placeholder: Handle<ColorMaterial>,
    tile: Handle<ColorMaterial>,
}

impl FromWorld for Materials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();

        Materials {
            board: materials.add(Color::rgb(0.7, 0.7, 0.8).into()),
            tile_placeholder: materials.add(Color::rgb(0.75, 0.75, 0.9).into()),
            tile: materials.add(Color::hsla(300.3, 1.0, 0.5, 0.5).into()),
        }
    }
}

struct Points {
    value: u32,
}

struct Position {
    x: u8,
    y: u8,
}

struct TileText;

struct FontSpec {
    family: Handle<Font>,
}

impl FromWorld for FontSpec {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();

        FontSpec {
            family: asset_server.load("fonts/FiraSans-Bold.ttf"),
        }
    }
}

enum BoardShift {
    Left,
    Right,
    Up,
    Down,
}

// KeyCode is an immutable reference because it's read-only in this context
impl TryFrom<&KeyCode> for BoardShift {
    type Error = &'static str;

    fn try_from(value: &KeyCode) -> Result<Self, Self::Error> {
        match value {
            KeyCode::Left => Ok(BoardShift::Left),
            KeyCode::Right => Ok(BoardShift::Right),
            KeyCode::Up => Ok(BoardShift::Up),
            KeyCode::Down => Ok(BoardShift::Down),
            _ => Err("not a valid board shift key"),
        }
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .init_resource::<Materials>()
        .init_resource::<FontSpec>()
        .add_startup_system(setup.system())
        .add_startup_system(spawn_board.system())
        .add_startup_system_to_stage(StartupStage::PostStartup, spawn_tiles.system())
        .add_system(render_tile_points.system())
        .add_system(board_shift.system())
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_board(mut commands: Commands, materials: Res<Materials>) {
    let board = Board::new(4);

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.board.clone(),
            sprite: Sprite::new(Vec2::new(board.physical_size, board.physical_size)),
            ..Default::default()
        })
        .with_children(|builder| {
            for tile in (0..board.size).cartesian_product(0..board.size) {
                builder.spawn_bundle(SpriteBundle {
                    material: materials.tile_placeholder.clone(),
                    sprite: Sprite::new(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    transform: Transform::from_xyz(
                        board.cell_position_to_physical(tile.0),
                        board.cell_position_to_physical(tile.1),
                        1.0,
                    ),
                    ..Default::default()
                });
            }
        })
        .insert(board);
}

fn spawn_tiles(
    mut commands: Commands,
    materials: Res<Materials>,
    query_board: Query<&Board>,
    font_spec: Res<FontSpec>,
) {
    let board = query_board
        .single()
        .expect("we always expect a board, panic if one does not exist");

    let mut random_number = rand::thread_rng();

    let starting_tiles: Vec<(u8, u8)> = (0..board.size)
        .cartesian_product(0..board.size)
        .choose_multiple(&mut random_number, 2);

    for (x, y) in starting_tiles.iter() {
        let position = Position { x: *x, y: *y };

        commands
            .spawn_bundle(SpriteBundle {
                material: materials.tile.clone(),
                sprite: Sprite::new(Vec2::new(TILE_SIZE, TILE_SIZE)),
                transform: Transform::from_xyz(
                    board.cell_position_to_physical(position.x),
                    board.cell_position_to_physical(position.y),
                    1.0,
                ),
                ..Default::default()
            })
            .with_children(|child_builder| {
                child_builder
                    .spawn_bundle(Text2dBundle {
                        text: Text::with_section(
                            "2",
                            TextStyle {
                                font: font_spec.family.clone(),
                                font_size: 40.0,
                                color: Color::BLACK,
                                ..Default::default()
                            },
                            TextAlignment {
                                vertical: VerticalAlign::Center,
                                horizontal: HorizontalAlign::Center,
                            },
                        ),
                        transform: Transform::from_xyz(0.0, 0.0, 1.0),
                        ..Default::default()
                    })
                    // Insert unit struct so we can filter for TileText in
                    // future queries
                    .insert(TileText);
            })
            .insert(Points { value: 2 })
            .insert(position);
    }
}

fn render_tile_points(
    mut texts: Query<&mut Text, With<TileText>>,
    // Points and Children are read-only in this function
    tiles: Query<(&Points, &Children)>,
) {
    for (points, children) in tiles.iter() {
        if let Some(entity) = children.first() {
            let mut text = texts.get_mut(*entity).expect("expected Text to exist");
            let mut text_section = text
                .sections
                .first_mut()
                .expect("expected first section to be available as mutable");
            text_section.value = points.value.to_string()
        }
    }
}

fn board_shift(keyboard_input: Res<Input<KeyCode>>) {
    let shift_direction = keyboard_input
        .get_just_pressed()
        .find_map(|key_code| BoardShift::try_from(key_code).ok());

    match shift_direction {
        Some(BoardShift::Left) => {
            dbg!("left");
        }
        Some(BoardShift::Right) => {
            dbg!("right");
        }
        Some(BoardShift::Up) => {
            dbg!("up");
        }
        Some(BoardShift::Down) => {
            dbg!("down");
        }
        None => (),
    }
}
