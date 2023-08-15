// use bevy::input::mouse::MouseButtonInput;
// use bevy::input::ButtonState;
// use bevy::sprite::MaterialMesh2dBundle;
// use bevy::transform::commands;
// use bevy::utils::tracing::field;
use bevy::window::PrimaryWindow;
use bevy::{prelude::*, winit::WinitSettings};
use starknet::core::types::{BlockId, BlockTag, FieldElement};
use std::str::FromStr;

// use rock_paper_scissor::dojo_utils::*;
use rock_paper_scissor::dojo::*;
use rock_paper_scissor::{configs::*, dojo_utils, game_data};

fn main() {
    App::new()
        //resources
        .insert_resource(WinitSettings::desktop_app()) // this makes sure the game only runs when the window is in focus to save on resources ... commented for possible incompatibility with linux
        //events
        //plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(DojoPlugin)
        //startup systems
        .add_startup_system(setup_bevy_game)
        //systems
        .add_system(button_system)
        .add_system(player_text_system)
        .add_system(dojo_button_system)
        .add_system(game_text_system)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup_bevy_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    commands.spawn(Camera2dBundle::default());

    let player_one = commands
        .spawn(game_data::Player {
            choice: game_data::Choice::None,
            crypto_address: game_data::CryptoAddress {
                address: configuration_values::ACCOUNT_ADDRESS_PLAYER_ONE.to_string(),
                secret: configuration_values::ACCOUNT_SECRET_KEY_PLAYER_ONE.to_string(),
            },
        })
        .id();
    let player_two = commands
        .spawn(game_data::Player {
            choice: game_data::Choice::None,
            crypto_address: game_data::CryptoAddress {
                address: configuration_values::ACCOUNT_ADDRESS_PLAYER_TWO.to_string(),
                secret: configuration_values::ACCOUNT_SECRET_KEY_PLAYER_TWO.to_string(),
            },
        })
        .id();

    commands.spawn(game_data::Game {
        player1: player_one,
        player2: player_two,
        outcome: game_data::Outcome::None,
    });

    let window = window_query.get_single().unwrap();
    let window_width = window.width();

    setup_buttons(player_one, window_width / 4.0, &mut commands, &asset_server);
    setup_buttons(
        player_two,
        (window_width / 4.0) * 3.0,
        &mut commands,
        &asset_server,
    );

    setup_game_dojo_ui(
        window_width / 2.0,
        window.height() - 200.0,
        &mut commands,
        &asset_server,
    );

    // spawn the text for each player here
}

fn setup_buttons(
    player_entity: Entity,
    x_coord: f32,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let button_spacing = 70.0; // Define a spacing between buttons
    let button_width = 175.0;
    let button_height = 65.0;

    let y_coord_basis = 20.0;

    let mut text_coord = 0.0;
    let text_coord_spacing = 40.0;

    let button_center = x_coord - (button_width / 2.0);

    for (i, &choice) in [
        game_data::Choice::Rock,
        game_data::Choice::Paper,
        game_data::Choice::Scissors,
    ]
    .iter()
    .enumerate()
    {
        let y_coord = y_coord_basis + (i as f32 * (button_height + button_spacing));
        text_coord = y_coord + button_height + text_coord_spacing;
        commands
            .spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: (Val::Px(button_center)),
                        top: (Val::Px(y_coord)),
                        right: (Val::Px(200.0)),
                        bottom: (Val::Px(200.0)),
                    },
                    size: Size::new(Val::Px(button_width), Val::Px(button_height)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|parent| {
                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(button_width), Val::Px(button_height)),
                            border: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        background_color: configuration_values::NORMAL_BUTTON.into(),
                        ..Default::default()
                    })
                    .insert(game_data::ButtonPlayerData {
                        // Attach the ButtonPlayerData component here.
                        player: player_entity,
                        choice: choice,
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            format!("{:?}", choice),
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ));
                    });
            });
    }

    setup_text(player_entity, x_coord, text_coord, commands, asset_server);
}

fn setup_text(
    player_entity: Entity,
    x_coord: f32,
    y_coord: f32,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let text_content = "Choice of player '{       }' is {      }";
    let avg_char_width = 10.0;
    let estimated_width = estimate_text_width(text_content, avg_char_width);

    // Adjust x_coord based on the estimated width
    let adjusted_x_coord = x_coord - estimated_width / 2.0;

    // Text with multiple sections
    commands.spawn((
        TextBundle::from_section(
            text_content,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(adjusted_x_coord),
                top: Val::Px(y_coord),
                ..Default::default() // Ensure other fields are set to their defaults
            },
            ..default()
        }),
        game_data::TextPlayerData {
            player: player_entity,
        },
    ));
}

fn setup_game_dojo_ui(
    x_coord: f32,
    y_coord: f32,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let text_content = "Outcome of the game is '{             }'";
    let avg_char_width = 10.0;
    let estimated_width = estimate_text_width(text_content, avg_char_width);

    // Adjust x_coord based on the estimated width
    let adjusted_x_coord = x_coord - estimated_width / 2.0;

    // Text with multiple sections
    commands.spawn((
        TextBundle::from_section(
            text_content,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(adjusted_x_coord),
                top: Val::Px(y_coord),
                ..Default::default() // Ensure other fields are set to their defaults
            },
            ..default()
        }),
        game_data::TextGameOutcome {},
    ));

    let button_width = 450.0;
    let button_height = 65.0;

    let button_center = x_coord - (button_width / 2.0);

    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: (Val::Px(button_center)),
                    top: (Val::Px(y_coord + button_height + 20.0)),
                    right: (Val::Px(200.0)),
                    bottom: (Val::Px(200.0)),
                },
                size: Size::new(Val::Px(button_width), Val::Px(button_height)),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(button_width), Val::Px(button_height)),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: configuration_values::NORMAL_BUTTON.into(),
                    ..Default::default()
                })
                .insert(game_data::TextGameOutcome {})
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        format!("Evaluate Outcome"),
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

fn game_text_system(
    mut text_data_query: Query<(&game_data::TextGameOutcome, &mut Text)>,
    game_data_query: Query<&game_data::Game>,
) {
    let game_data = game_data_query.get_single().unwrap();

    for (_text_data, mut text) in text_data_query.iter_mut() {
        match game_data.outcome {
            game_data::Outcome::Player1Wins => {
                text.sections[0].value = "Outcome of the game is 'Player 1 wins'".to_string();
                text.sections[0].style.color = Color::rgb(0.0, 1.0, 0.0);
            }
            game_data::Outcome::Player2Wins => {
                text.sections[0].value = "Outcome of the game is 'Player 2 wins'".to_string();
                text.sections[0].style.color = Color::rgb(0.0, 1.0, 0.0);
            }
            game_data::Outcome::Tie => {
                text.sections[0].value = "Outcome of the game is 'a tie'".to_string();
                text.sections[0].style.color = Color::rgb(1.0, 0.5, 0.0);
            }
            game_data::Outcome::None => {
                text.sections[0].value = "Outcome of the game is 'Undecided'".to_string();
                text.sections[0].style.color = Color::rgb(1.0, 0.0, 0.0);
            }
        }
    }
}

/////////////////////////////////////////////////////////////////
//////////// Systems (functions called on update) ///////////////
/////////////////////////////////////////////////////////////////

fn dojo_button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &Children,
            &game_data::TextGameOutcome,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    check_game_component: Res<CheckGame>,
) {
    for (interaction, mut color, children, _button_data) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();

        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Sending Evaluation call".to_string();
                *color = configuration_values::PRESSED_BUTTON.into();

                if let Err(e) = check_game_component.try_send() { // this one works fine
                     //log::error!("updating the players: {e}");
                }

                println!("Evaluate Outcome");
            }
            Interaction::None => {
                text.sections[0].value = format!("Click to ask for evaluation");
                *color = configuration_values::NORMAL_BUTTON.into();
            }
            _ => {}
        }
    }
}

fn player_text_system(
    mut text_data_query: Query<(&game_data::TextPlayerData, &mut Text)>,
    player_query: Query<&game_data::Player>,
) {
    for (text_data, mut text) in text_data_query.iter_mut() {
        if let Ok(player) = player_query.get(text_data.player) {
            let player_name =
                dojo_utils::slice_string(player.crypto_address.address.to_string(), 8, false); // You might need a way to fetch the player's name
            let choice_string = format!("{:?}", player.choice); // Convert choice enum to string

            text.sections[0].value =
                format!("Choice of player '{}' is {}", player_name, choice_string);
        }
    }
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &Children,
            &game_data::ButtonPlayerData,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut players: Query<&mut game_data::Player>,
    update_choice: Res<UpdateChoices>,
) {
    for (interaction, mut color, children, button_data) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();

        // Query the player component for the player_entity
        if let Ok(mut player) = players.get_mut(button_data.player) {
            match *interaction {
                Interaction::Clicked => {
                    text.sections[0].value = "Choice set".to_string();
                    *color = configuration_values::PRESSED_BUTTON.into();
                    player.choice = button_data.choice;

                    let game_address = FieldElement::from_str(&player.crypto_address.address).unwrap();
                    let player_choice =   FieldElement::from(player.choice as u32)   ;

                    if let Err(e) = update_choice.try_send(game_address, player_choice) {
                        //log::error!("updating the choice: {:?}", e);
                    }
                }
                Interaction::None => {
                    text.sections[0].value = format!("{:?}", button_data.choice);
                    *color = configuration_values::NORMAL_BUTTON.into();
                }
                _ => {}
            }
        }
    }
}

/////////////////////////////////////////////
//////////// Helper Functions ///////////////
/////////////////////////////////////////////

fn estimate_text_width(text: &str, avg_char_width: f32) -> f32 {
    text.chars().count() as f32 * avg_char_width
}
