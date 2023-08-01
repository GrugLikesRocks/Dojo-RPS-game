use std::f32::consts::E;

use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use bevy::window::PrimaryWindow;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
    None,
}
#[derive(Component)]
pub struct Game {
    pub started: bool,
}

#[derive(Debug)]
pub struct Rect {
    choice: Choice,
    text: String,
    size: Vec2,
    position: Vec2,
}

impl Rect {
    fn contains_point(&self, point: Vec2) -> bool {
        let min = self.position;
        let max = Vec2::new(self.position.x + self.size.x, self.position.y + self.size.y);


        if (point.x > min.x && max.x > point.x) && (point.y > min.y && max.y > point.y) {
            println!("\n\n");
            true
        } else {
            false
        }
    }
}

#[derive(Component)]
pub struct Player {
    pub choice: Choice,
    pub rects: Vec<Rect>,
    pub player_num: u8,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins))
        .add_startup_system(setup)
        .add_system(spawn_rects_per_player)
        .add_system(mouse_click_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(Game { started: false });

    commands.spawn(Player {
        choice: Choice::None,
        rects: Vec::new(),
        player_num: 0,
    });

    commands.spawn(Player {
        choice: Choice::None,
        rects: Vec::new(),
        player_num: 1,
    });

}

fn spawn_rects_per_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut players_query: Query<&mut Player>,
    mut game: Query<&mut Game>,
) {
    let mut game_state = game.single_mut();
    if game_state.started == false {
        let choices = [Choice::Rock, Choice::Paper, Choice::Scissors];
        let positions_fake = [-300.0, 300.0];

        let mut x = 0; // for the fake position
        let mut y = 0; // for the shitty position
        let mut z = 0; //for the colour

        let positions = vec![
            Vec2::new(315.0, 190.0),
            Vec2::new(315.0, 310.0),
            Vec2::new(315.0, 430.0),
            Vec2::new(915.0, 190.0),
            Vec2::new(915.0, 310.0),
            Vec2::new(915.0, 430.0),
        ];

        let colors = vec![
            Color::rgb(0.65, 0.16, 0.16),
            Color::rgb(0.33, 0.33, 0.33), // Dark Gray
            Color::rgb(1.0, 1.0, 1.0),    // White
        ];

        for mut player in players_query.iter_mut() {
            for (i, &choice) in choices.iter().enumerate() {
                let size = Vec2::new(50.0, 100.0);
                let position = positions[y];
                let fake_position = Vec2::new(positions_fake[x] as f32, i as f32 * 120.0 - 120.0);
                let color = colors[z];
                let text = format!("{:?}", choice);

                let rect = Rect {
                    text,
                    choice,
                    size,
                    position,
                };
                player.rects.push(rect);

                let bundle = MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Quad::new(size).into()).into(),
                    material: materials.add(ColorMaterial::from(color)),
                    transform: Transform::from_translation(Vec3::new(
                        fake_position.x,
                        fake_position.y,
                        0.0,
                    )),
                    ..Default::default()
                };
                commands.spawn(bundle);

                y = y + 1;
                z = z + 1
            }
            z = 0;
            x = x + 1;
        }
        game_state.started = true;
    }
}

fn mouse_click_system(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    players_query: Query<&Player>,
) {
    let window = window_query.get_single().unwrap();

    for event in mouse_button_input_events.iter() {
        if event.state == ButtonState::Pressed {
            if let Some(mut position) = window.cursor_position() {
                // Flip the y-coordinate
                position.y = (window.height() - position.y);
                position.x = position.x;

                for player in players_query.iter() {
                    for rect in &player.rects {
                        if rect.contains_point(position) {
                            println!("Player clicked on {:?}", rect.choice);
                            player_choice(player.player_num, rect.choice);
                        }
                    }
                }
            }
        }
    }
}

fn player_choice(player_num: u8, choice: Choice) {
    println!("Player {} chose {:?}", player_num, choice);
}

