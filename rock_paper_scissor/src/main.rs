use std::f32::consts::E;

use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::{
    prelude::*,
    sprite::Anchor,
    text::{BreakLineOn, Text2dBounds},
};
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

#[derive(Debug)]

pub struct TextStruct {
    text: String,
    position: Vec3,
    entity: Option<Entity>,
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
    pub text_entity: Option<Entity>,
}

pub struct TextChoiceEvent
{
    pub player_id : u8,
    pub choice : Choice,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins))
        .add_event::<TextChoiceEvent>()
        .add_startup_system(setup)
        .add_system(spawn_rects_per_player)
        .add_system(mouse_click_system)
        .add_system(player_choice_event)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(Game { started: false });

    commands.spawn(Player {
        choice: Choice::None,
        rects: Vec::new(),
        player_num: 1,
        text_entity: None,
    });

    commands.spawn(Player {
        choice: Choice::None,
        rects: Vec::new(),
        player_num: 2,
        text_entity: None,
    });

}

fn spawn_rects_per_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut players_query: Query<&mut Player>,
    mut game: Query<&mut Game>,
    asset_server: Res<AssetServer>,
) {
    let mut game_state = game.single_mut();
    if game_state.started == false {
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        let text_style = TextStyle {
            font: font.clone(),
            font_size: 60.0,
            color: Color::WHITE,
        };
        let text_alignment = TextAlignment::Center;

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
            if x == 0 {
                let entity_id = commands
                    .spawn((Text2dBundle {
                        text: Text::from_section("player 1 choice", text_style.clone())
                            .with_alignment(text_alignment),
                        transform: Transform::from_translation(Vec3 {
                            x: (-450 as f32),
                            y: (-250 as f32),
                            z: (0 as f32),
                        }),
                        ..default()
                    },))
                    .id();

                player.text_entity = Some(entity_id);
            } 
            else {
                let entity_id = commands
                    .spawn((Text2dBundle {
                        text: Text::from_section("player 2 choice", text_style.clone())
                            .with_alignment(text_alignment),
                        transform: Transform::from_translation(Vec3 {
                            x: (350 as f32),
                            y: (-250 as f32),
                            z: (0 as f32),
                        }),
                        ..default()
                    },))
                    .id();

                player.text_entity = Some(entity_id);
            }

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


                commands
                    .spawn((Text2dBundle {
                        text: Text::from_section(format!("{:?}", choice), text_style.clone())
                            .with_alignment(text_alignment),
                            transform: Transform::from_translation(Vec3::new(
                                fake_position.x,
                                fake_position.y,
                                0.0,
                            )),
                        ..default()
                    },));

                y = y + 1;
                z = z + 1;
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
    mut text_choice_eventwriter: EventWriter<TextChoiceEvent>,
) {
    let window = window_query.get_single().unwrap();

    for event in mouse_button_input_events.iter() {
        if event.state == ButtonState::Pressed {
            if let Some(mut position) = window.cursor_position() {

                for player in players_query.iter() {
                    for rect in &player.rects {
                        if rect.contains_point(position) {
                            //println!("Player clicked on {:?}", rect.choice);
                            player_choice(player.player_num, rect.choice);
                            text_choice_eventwriter.send(TextChoiceEvent { player_id: (player.player_num), choice: (rect.choice) })
                        }
                    }
                }
            }
        }
    }
}



fn player_choice_event(
    mut text_choice_eventreader: EventReader<TextChoiceEvent>,
    mut players_query: Query<&mut Player>,
    mut texts: Query<&mut Text>,
) {

    for event in text_choice_eventreader.iter() {
        for mut player in players_query.iter_mut() {
            if player.player_num == event.player_id {
                player.choice = event.choice;
                print!("Player {} chose dddddd{:?}", player.player_num, player.choice);
                

                if let Some(text_entity) = player.text_entity {
                    if let Ok(mut text) = texts.get_mut(text_entity) {
                        text.sections[0].value = format!("Player {} chose {:?}", player.player_num, player.choice);
                    }
                }
            }
        }
    }
}


fn player_choice(player_num: u8, choice: Choice) {
    println!("Player {} chose {:?}", player_num, choice);
}
