use bevy::window::PrimaryWindow;
use bevy::{prelude::*, winit::WinitSettings};
use starknet::core::types::FieldElement;
use std::str::FromStr;

use rock_paper_scissor::dojo::*;
use rock_paper_scissor::{configs::*, dojo_utils, game_data};

fn main() {
    App::new()
        //resources
        .insert_resource(WinitSettings::desktop_app()) // this makes sure the game only runs when the window is in focus to save on resources
        //events
        //plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(DojoPlugin) // custom plugin made from the dojo.rs script
        //startup systems: this run at the start of the game once
        .add_startup_system(setup_bevy_game)
        //systems:  these runs every frame
        .add_system(player_button_system)
        .add_system(player_text_system)
        .add_system(dojo_button_system)
        .add_system(game_text_system)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup_bevy_game(
    mut commands: Commands, // commands is the way to spawn entities in bevy
    asset_server: Res<AssetServer>, // asset_server is the way to load assets in bevy, the folder is also called assets and in this case its for the font
    window_query: Query<&Window, With<PrimaryWindow>>, // this is a query to get the window
) {
    commands.spawn(Camera2dBundle::default()); // this spawns a camera

    // spanw a player entity with the component player and then saving it in a variable
    let player_one = commands
        .spawn(game_data::Player {
            choice: game_data::Choice::None,

            crypto_address: game_data::CryptoAddress {
                // component that holds the address and secret key of the player
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

    //spawn a game component with the player entities and the outcome enum type of the game
    commands.spawn(game_data::Game {
        player1: player_one,
        player2: player_two,
        outcome: game_data::Outcome::None,
    });

    // get the window               we know there is only one window so we can use get_single        unwrap is used to get the value out of the option     in case of issues it will panic
    let window = window_query.get_single().unwrap();

    let window_width = window.width();

    // these functions could also be called as startup_systems but its just to keep things neat.
    // we send references to everything necessary for the spawning and the setup of the ui
    setup_player_buttons(player_one, window_width / 4.0, &mut commands, &asset_server);
    setup_player_buttons(
        player_two,
        (window_width / 4.0) * 3.0,
        &mut commands,
        &asset_server,
    );

    // setup the button and the text of the game outcome
    setup_game_dojo_ui(
        window_width / 2.0,
        window.height() - 200.0,
        &mut commands,
        &asset_server,
    );
}

// this function spawns 3 buttons per player for the rock paper scissor choice and then a text component to show the choice
fn setup_player_buttons(
    player_entity: Entity,
    x_coord: f32,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let button_spacing = 70.0; // Define a spacing between buttons

    let button_width = 175.0; // button size
    let button_height = 65.0;

    let y_coord_basis = 20.0; // start y coord of the position of the buttons

    let mut text_coord = 0.0; // start y coord of the position of the text, this will get added on as the buttons spawn hence the mut as its mutable and we are changing it
    let text_coord_spacing = 40.0; // spacing between the text and the buttons

    let button_center = x_coord - (button_width / 2.0);

    // Iterating through the list of game choices (Rock, Paper, Scissors)
    // with their respective indices (i) and values (choice)
    for (i, &choice) in [
        game_data::Choice::Rock,
        game_data::Choice::Paper,
        game_data::Choice::Scissors,
    ]
    .iter()
    .enumerate()
    {
        // Calculating the y-coordinate for each button based on its index in the list
        // This allows for positioning the buttons vertically with spacing in between
        let y_coord = y_coord_basis + (i as f32 * (button_height + button_spacing));

        // Calculating the y-coordinate for the text within the button
        text_coord = y_coord + button_height + text_coord_spacing;

        // Spawning a node (probably a UI container) with a specific position and size
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
                // Within this node, a button is spawned with specific styling and attributes
                parent
                    .spawn(ButtonBundle { // a bundle is a something that is from bevy and is a collection of components
                        // in this case we are changing the style comp and the background comp
                        // style is very simial to css

                        // here are all the comps in this case  https://docs.rs/bevy/0.10.1/bevy/ui/node_bundles/struct.ButtonBundle.html
                        style: Style {
                            size: Size::new(Val::Px(button_width), Val::Px(button_height)),
                            border: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },

                        // Setting the button's background color to a predefined value
                        background_color: configuration_values::NORMAL_BUTTON.into(),

                        // once we changed everything we need we can just leave everything else as default
                        ..Default::default()
                    })

                    .insert(game_data::ButtonPlayerData {
                        // Attaching additional data to the button to represent the player and the game choice
                        // this will be needed as we are going to query the button with the player component so we get the right one
                        // also its necessary to know which player owns this button
                        player: player_entity,
                        choice: choice,
                    })

                    .with_children(|parent| {
                        // Spawning a text element as a child of the button
                        // This text displays the name of the game choice (Rock, Paper, or Scissors)
                        parent.spawn(TextBundle::from_section(
                            //samething as the button with the comps
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

    setup_player_text(player_entity, x_coord, text_coord, commands, asset_server);
}

fn setup_player_text(
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
                ..Default::default() // Ensure other comps are set to their defaults
            },
            ..default()
        }),
        game_data::TextPlayerData {
            player: player_entity,
        },
    ));
}



//spawns the button and the text for the game outcome
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
        game_data::UIGameOutcome {},  //adding the empty component to the text so we can query it later knowing this it the right text
        //same thing for the button below
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

                .insert(game_data::UIGameOutcome {})

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




/////////////////////////////////////////////////////////////////
//////////// Systems (functions called on update) ///////////////
/////////////////////////////////////////////////////////////////

//called every update to change the text and colour of the text gmae outcome 
fn game_text_system( 
    mut text_data_query: Query<(&game_data::UIGameOutcome, &mut Text)>, //gets the entity with the text component and the UIGameOutcome component only
    game_data_query: Query<&game_data::Game>,  //gets the current game data
) {
    let game_data = game_data_query.get_single().unwrap();  // we know there is only one game data so we can use get_single

    for (_text_data, mut text) in text_data_query.iter_mut() {    //theoretically there is only one text data but we are using a for loop just in case
        // we split the components of the query in two, variables needed. we dont really need the UIGameOutcome so we add a _ before the name so rust shuts up
        match game_data.outcome {  //change the text and color of the text based on the outcome of the game
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


fn dojo_button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &Children,
            &game_data::UIGameOutcome,
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

                if let Err(_e) = check_game_component.try_send() { // this one works fine
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


// updates the text for the players once they have chosen a new choice, should preferably be an event based system instead of an update system
// as we only need to update it when the player has chosen a new choice
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

fn player_button_system(
    mut interaction_query: Query<   //usual query to get all the component we need from the button
        (
            &Interaction,
            &mut BackgroundColor,
            &Children,
            &game_data::ButtonPlayerData,
        ),
        (Changed<Interaction>, With<Button>),   // but here we give two params, the second one checks if the button has had its interactions changed, this is some backend stuff with bevy
        // but for example instead of getting the 6 buttons we get from the 6 buttons the ones that have their state changed whetehr it is from the 
        // user hovering or clicking on it
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
                Interaction::Clicked => {  // if the button is clicked we change the text and color of the button and we update the player choice
                    text.sections[0].value = "Choice set".to_string();
                    *color = configuration_values::PRESSED_BUTTON.into();
                    player.choice = button_data.choice;

                    let game_address = FieldElement::from_str(&player.crypto_address.address).unwrap();
                    let player_choice = FieldElement::from(player.choice as u32);
                    //set everything to fieldelemnts and then send them to update the dojo system by trying to make a call to one of the open channels in the dojo system
                    if let Err(_e) = update_choice.try_send(game_address, player_choice) {
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

// function that estimates the width of a text based on the number of characters and the average width of a character
// needed to center text
fn estimate_text_width(text: &str, avg_char_width: f32) -> f32 {
    text.chars().count() as f32 * avg_char_width
}
