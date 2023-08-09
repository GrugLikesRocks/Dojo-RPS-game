use std::f32::consts::E;

use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
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
            //println!("\n\n");
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
    pub player_field : FieldElement,
}

pub struct TextChoiceEvent
{
    pub player_id : u8,
    pub choice : Choice,
}


pub struct PlayerInitiatedEvent;



fn main() {


    let url = Url::parse(configs::JSON_RPC_ENDPOINT).unwrap();
    let account_address = FieldElement::from_str(configs::ACCOUNT_ADDRESS).unwrap();
    let account = SingleOwnerAccount::new(
            JsonRpcClient::new(HttpTransport::new(url)),
            LocalWallet::from_signing_key(SigningKey::from_secret_scalar(
                FieldElement::from_str(configs::ACCOUNT_SECRET_KEY).unwrap(),
            )),
            account_address,
            cairo_short_string_to_felt("KATANA").unwrap(),
    );

    let world_address = FieldElement::from_str(configs::WORLD_ADDRESS).unwrap();


    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<TextChoiceEvent>()
        .add_event::<PlayerInitiatedEvent>()
        .add_plugin(TokioTasksPlugin::default())
        .insert_resource(DojoEnv::new(world_address, account))
        .add_startup_system(setup)
        .add_startup_system(start_game_dojo)
        .add_system(spawn_rects_per_player)
        .add_system(mouse_click_system)
        .add_system(player_choice_event)
        .run();
}

fn setup(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(Game { started: false });

    commands.spawn(Player {
        choice: Choice::None,
        rects: Vec::new(),
        player_num: 1,
        text_entity: None,
        player_field: (1 as u8).into()  // this is retarded
    });

    commands.spawn(Player {
        choice: Choice::None,
        rects: Vec::new(),
        player_num: 2,
        text_entity: None,
        player_field: (2 as u8).into(),
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

                if let Some(text_entity) = player.text_entity {
                    if let Ok(mut text) = texts.get_mut(text_entity) {
                        text.sections[0].value = format!("Player {} chose {:?}", player.player_num, player.choice);
                    }
                }
            }
        }
    }
}


fn player_initiated_event(
    mut event: EventReader<PlayerInitiatedEvent>,
) {

    for e in event.iter() {
        println!("Player initiated event")
    }
}


fn player_choice(player_num: u8, choice: Choice) {
    //println!("Player {} chose {:?}", player_num, choice);
}


///////////////////////////////////////////////////
//////////////////// CAIRO SECTION ///////////////////////
//////////////////////////////////////////////////


use rock_paper_scissor::configs;

use bevy::ecs::system::SystemState;
use bevy::log;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tokio_tasks::TaskContext;
use bevy_tokio_tasks::{TokioTasksPlugin, TokioTasksRuntime};
use dojo_client::contract::world::WorldContract;
use num::bigint::BigUint;
use num::{FromPrimitive, ToPrimitive};
use rand::Rng;
use starknet::accounts::SingleOwnerAccount;
use starknet::core::types::{BlockId, BlockTag, FieldElement};
use starknet::core::utils::cairo_short_string_to_felt;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::JsonRpcClient;
use starknet::signers::{LocalWallet, SigningKey};
use std::ops::Div;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::mpsc;
use url::Url;






//all of dojo world stuff

fn setup_dojo(mut commands: Commands) {
    commands.spawn(DojoSyncTime::from_seconds(configs::DOJO_SYNC_INTERVAL));
}


#[derive(Component)]
struct DojoSyncTime {
    timer: Timer,
}
// this has somethign to do with an update ticker
impl DojoSyncTime {
    fn from_seconds(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Repeating),
        }
    }
}



//this on the other hand is called eveyr update, by the looks of things this is to update the dt on the dojo side
//but ofcourse to update you need a reference and i think its those resource 

//for now commented out but should be the thing that checks when to win or lose
// fn sync_dojo_state(
//     mut dojo_sync_time: Query<&mut DojoSyncTime>,
//     time: Res<Time>,
//     spawn_racers: Res<StartGameCommand>,   //
// ) {
//     let mut dojo_time = dojo_sync_time.single_mut();
//     // This retrieves a mutable reference to the DojoSyncTime component or resource from the ECS world.
//     // this is a bevy thing


//     if dojo_time.timer.just_finished() {
//         dojo_time.timer.reset();
//         //If the timer inside dojo_time has just finished its countdown, it's being reset.
//         if cars.is_empty() {// if there are no then spawn some
//             if let Err(e) = spawn_racers.try_send() {
//                 log::error!("Spawn racers channel: {e}");
//             }
//         } else { //else tick
//             if let Err(e) = update_vehicle.try_send() {
//                 log::error!("Update vehicle channel: {e}");
//             }
//             if let Err(e) = drive.try_send() {
//                 log::error!("Drive channel: {e}");
//             }
//             if let Err(e) = update_enemies.try_send() {
//                 log::error!("Update enemies channel: {e}");
//             }

//             //tries to sent the message to the channel

//         }
//     } else {
//         dojo_time.timer.tick(time.delta());
//         //else make it tick
//     }
// }




// derive is a macro that automatically implements some traits for a struct basically an interface in c++
#[derive(Resource)]
pub struct DojoEnv {
    /// The block ID to use for all contract calls.
    block_id: BlockId,
    /// The address of the world contract.
    world_address: FieldElement,
    /// The account to use for performing execution on the World contract.
    account: Arc<SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>>,
}

// impl is a keyword that implements functions for a struct
impl DojoEnv {
    fn new(
        world_address: FieldElement,
        account: SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>,
    ) -> Self {
        Self {
            world_address,
            account: Arc::new(account),
            block_id: BlockId::Tag(BlockTag::Latest),
        }
    }
}




#[derive(Resource)]
pub struct StartGameCommand(mpsc::Sender<()>);

impl StartGameCommand {
    pub fn try_send(&self)
        // Result<T, E>: The Result type in Rust represents either a successful value of type T or an error of type E.
        // this is not a touple
     -> Result< (), mpsc::error::TrySendError<()> > {
        // i think its like if it does fail it saves the error in e

        self.0.try_send(())
    }
}











#[derive(Resource)]
pub struct CheckGame(mpsc::Sender<()>);

impl CheckGame {
    pub fn try_send(&self)
        // Result<T, E>: The Result type in Rust represents either a successful value of type T or an error of type E.
        // this is not a touple
     -> Result< (), mpsc::error::TrySendError<()> > {
        // i think its like if it does fail it saves the error in e

        self.0.try_send(())
    }
}

//some sort of retunr value from dojo?
pub struct UpdateCar {
    pub vehicle: Vec<FieldElement>,// vector of field elements
}





// this should spawn the players
fn start_game_dojo(
    env: Res<DojoEnv>,
    runtime: ResMut<TokioTasksRuntime>,
    mut commands: Commands,
) {
   
    let (tx, mut rx) = mpsc::channel::<()>(8);

    commands.insert_resource(StartGameCommand(tx));
 
    let account = env.account.clone();
    let world_address = env.world_address;
    let block_id = env.block_id;

    runtime.spawn_background_task(move |mut ctx| async move {

        let world = WorldContract::new(world_address, account.as_ref());

        let start_game_system = world.system("start_game_dojo_side", block_id).await.unwrap();

        let player_num_one = get_given_model_id(ctx.clone(), (1 as u8).into()).await.unwrap();

        let player_num_two = get_given_model_id(ctx.clone(), (1 as u8).into()).await.unwrap();

        while let Some(_) = rx.recv().await {

            match start_game_system
                .execute(vec![
                    player_num_one,
                ])
                .await
                //await is asyncrounous
            {
                Ok(_) => {
                    ctx.run_on_main_thread(move |ctx| 
                    {
                      
                        let mut state: SystemState<EventWriter<PlayerInitiatedEvent>> = SystemState::new(ctx.world);
                              
                        let mut spawn_player= state.get_mut(ctx.world);
                         
                        spawn_player.send(PlayerInitiatedEvent); 
                        
                    })
                    .await;
                    
                }
                Err(e) => {
                    log::error!("Run spawn_player system: {e}");
                }
            }

            match start_game_system
                .execute(vec![
                    player_num_two,
                ])
                .await
                //await is asyncrounous
            {
                Ok(_) => {
                    ctx.run_on_main_thread(move |ctx| 
                    {
                        let mut state: SystemState<EventWriter<PlayerInitiatedEvent>> = SystemState::new(ctx.world);
                              
                        let mut spawn_player= state.get_mut(ctx.world);
                         
                        spawn_player.send(PlayerInitiatedEvent); 
                        
                    })
                    .await;
                    
                }
                Err(e) => {
                    log::error!("Run spawn_player system: {e}");
                }
            }
        }
    });
}





// // this is apparently a startup system so only called once but it has an indefinite loop in the middle, thats where the update happens
// fn update_vehicle_thread(
//     env: Res<DojoEnv>,
//     runtime: ResMut<TokioTasksRuntime>,
//     mut commands: Commands,
// ) {

//     let (tx, mut rx) = mpsc::channel::<()>(16);

//     commands.insert_resource(CheckGame(tx));

//     let account = env.account.clone();
//     let world_address = env.world_address;
//     let block_id = env.block_id;

//     runtime.spawn_background_task(move |mut ctx| async move {

//         let world = WorldContract::new(world_address, account.as_ref());

//         //get the component/struct Player from the dojo world
//         let player_component = world.component("Player", block_id).await.unwrap();

//         //this is the loop that runs indefinetly
//         while let Some(_) = rx.recv().await {
           
//             //let model_id = get_model_id(ctx.clone()).await;
            

          
//                 match player_component
//                     .entity(FieldElement::ZERO, vec![model_id], block_id)
//                     .await
//                 {   // the return data is then saved in the vehicle variable
//                     Ok(vehicle) => 
//                     {
//                         ctx.run_on_main_thread(move |ctx| 
//                         {   
//                             let mut state: SystemState<EventWriter<UpdateCar>> = SystemState::new(ctx.world);

//                             let mut update_car = state.get_mut(ctx.world);
              
//                             update_car.send(UpdateCar { vehicle })  

//                         })
//                         .await;
//                     }

//                     Err(e) => {
//                         log::error!("Query `Vehicle` component: {e}");
//                     }
//                 }
            
//         }
//     });
// }





async fn get_given_model_id(mut ctx: TaskContext, given_id: FieldElement) -> Option<FieldElement> {
    ctx.run_on_main_thread(move |_ctx| {
        
        Some(given_id)
    })
    .await
}
