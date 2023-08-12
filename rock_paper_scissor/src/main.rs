use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::utils::tracing::field;
use bevy::window::PrimaryWindow;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Choice {
    Rock  =3,
    Paper =2,
    Scissors=1,
    None=0,
}
#[derive(Component)]
pub struct Game {
    pub started: bool,
    pub dojo_game: bool,
    pub current_game_state: u8,
}

#[derive(Debug,Clone)]
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

#[derive(Component,Clone)]
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
        .add_event::<CheckGame>()
        .add_event::<GameUpdate>()

        .add_plugin(TokioTasksPlugin::default())

        .insert_resource(DojoEnv::new(world_address, account))

        .add_startup_system(setup)
        .add_startup_system(setup_dojo.after(setup))
        .add_startup_system(start_game_dojo.after(setup_dojo))
        .add_startup_system(fetch_game_component.after(start_game_dojo))
        .add_startup_system(update_choices_thread.after(fetch_game_component))
        .add_startup_system(set_winner_state_thread.after(update_choices_thread))

        .add_system(sync_dojo_state)
        .add_system(spawn_rects_per_player.after(sync_dojo_state))
        .add_system(mouse_click_system.after(sync_dojo_state))
        .add_system(player_choice_event.after(sync_dojo_state))
        .add_system(player_initiated_event.after(sync_dojo_state))
        .add_system(check_game_update.after(sync_dojo_state))
        .run();
}

fn setup(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(Game { started: false,dojo_game: false, current_game_state: 0 });

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
use tokio::sync::mpsc::Receiver;




const PLAYER_NUM_ONE: u8 = 1 ;

const PLAYER_NUM_TWO: u8 = 2 ;


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

// the issue is that the players are not techincally spawned in the real world


fn sync_dojo_state(
    mut players_query: Query<(&Player,)>,
    mut dojo_sync_time: Query<&mut DojoSyncTime>,
    mut game: Query<&mut Game>,
    time: Res<Time>,
    spawn_players: Res<StartGameCommand>,   //
    check_game_component: Res<CheckGame>,
    update_choice: Res<UpdateChoices>,
    change_game_state: Res<UpdateGameWinnerState>
) {
    let mut dojo_time = dojo_sync_time.single_mut();
 

    let mut game_state = game.single_mut();
    if dojo_time.timer.just_finished() {
        dojo_time.timer.reset();
        
        if game_state.dojo_game == false {

            println!("Game started");

            if let Err(e) = spawn_players.try_send() {  // this one works fine
                log::error!("Spawn players channel: {e}");
            }


            game_state.dojo_game = true;

        } else { 
            if let Err(e) = check_game_component.try_send() {     // this one works fine
                log::error!("updating the players: {e}");
            }



               // Fetch player choices here since sync_dojo_state is called every frame
               let mut player1_choice: i32 = 0; // default values
               let mut player2_choice: i32 = 0;
   
               for player in players_query.iter() {
                   match player.0.player_num {
                       1 => player1_choice = player.0.choice as i32,
                       2 => player2_choice = player.0.choice as i32,
                       _ => {}
                   }
               }
   


            if let Err(e) = update_choice.try_send(player1_choice, player2_choice) {
                log::error!("updating the choice: {:?}", e);
            }
            // if let Err(e) = update_choice.try_send() {
            //     log::error!("updating the choice: {e}");
            // }

            if let Err(e) = change_game_state.try_send() {   // thsi one also works fine
                log::error!("updating the game state: {e}");
            }
        }   
    } else {
        dojo_time.timer.tick(time.delta()); 
    }
}




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



// these spawns the players at the start
fn start_game_dojo(
    env: Res<DojoEnv>,
    runtime: ResMut<TokioTasksRuntime>,
    mut commands: Commands,
) {
   
    let (tx, mut rx) = mpsc::channel::<()>(8);

    commands.insert_resource(StartGameCommand(tx));
    
    println!("start game dojo");

    let account = env.account.clone();
    let world_address = env.world_address;
    let block_id = env.block_id;

    runtime.spawn_background_task(move |mut ctx| async move {


        let world = WorldContract::new(world_address, account.as_ref());

        let start_game_system = world.system("start_game_dojo_side", block_id).await.unwrap();

        while let Some(_) = rx.recv().await {
            match start_game_system
                .execute(vec![
                    PLAYER_NUM_ONE.into(),
                    into_field_element(10)
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

        println!("start game dojo async after while loop");
    });
}










// #[derive(Resource)]
// pub struct UpdateChoices(mpsc::Sender<()>);

// impl UpdateChoices {
//     pub fn try_send(&self)
//         // Result<T, E>: The Result type in Rust represents either a successful value of type T or an error of type E.
//         // this is not a touple
//      -> Result< (), mpsc::error::TrySendError<()> > {
//         // i think its like if it does fail it saves the error in e

//         self.0.try_send(())
//     }
// }







#[derive(Debug, Clone)]
pub struct PlayerChoiceData {
   pub player1_choice: i32,
   pub player2_choice: i32,
}

// Updated UpdateChoices to send PlayerChoiceData
#[derive(Resource)]
pub struct UpdateChoices(mpsc::Sender<PlayerChoiceData>);

impl UpdateChoices {
    pub fn try_send(&self, player1_choice: i32, player2_choice: i32)
     -> Result<(), mpsc::error::TrySendError<PlayerChoiceData>> {
        self.0.try_send(PlayerChoiceData {
            player1_choice,
            player2_choice,
        })
    }
}



// this sends the new choices of the player
fn update_choices_thread(
    env: Res<DojoEnv>,
    runtime: ResMut<TokioTasksRuntime>,
    mut commands: Commands,
) {
    let (tx, mut rx) = mpsc::channel::<PlayerChoiceData>(8);
    
    commands.insert_resource(UpdateChoices(tx));

    let account = env.account.clone();
    let world_address = env.world_address;
    let block_id = env.block_id;

    runtime.spawn_background_task(move |mut ctx| async move {
        let world = WorldContract::new(world_address, account.as_ref());
        let update_choice_system = world.system("update_player_choice", block_id).await.unwrap();

        while let Some(data) = rx.recv().await {
            match update_choice_system
                .execute(vec![
                    into_field_element(data.player1_choice as u8),
                    into_field_element(data.player2_choice as u8),
                ])
                .await
            {
                Ok(_) => {
                    ctx.run_on_main_thread(move |_ctx| {
                        println!("call sent fine");
                    })
                    .await;
                }
                Err(e) => {
                    log::error!("Run update choice system: {:?}", e);
                }
            }
        }
    });
}


// pub struct UpdatePlayerChoiceEvent {
//     pub player_num: u8,
//     pub choice: Choice,
//     // ... any other data you need
// }

// fn update_choices_thread(
//     env: Res<DojoEnv>,
//     runtime: ResMut<TokioTasksRuntime>,
//     mut commands: Commands,
// ) {
//     // Changed the type of the channel to handle UpdatePlayerChoiceEvent
//     let (tx, mut rx) = mpsc::channel::<>(8);
    
//     commands.insert_resource(UpdateChoices(tx));

//     let account = env.account.clone();
//     let world_address = env.world_address;
//     let block_id = env.block_id;

//     runtime.spawn_background_task(move |mut ctx| async move {

//         let world = WorldContract::new(world_address, account.as_ref());

//         let update_choice_system = world.system("update_player_choice", block_id).await.unwrap();

//         // Changed the loop to handle UpdatePlayerChoiceEvent received from the channel
//         while let Some(event) = rx.recv().await {

//             match update_choice_system
//                 .execute(vec![
//                     into_field_element(event.player_num), 
//                     into_field_element(event.choice as u8), 
//                     // ... other data if needed
//                 ])
//                 .await
//             {
//                 Ok(_) => {
//                     ctx.run_on_main_thread(move |_ctx| 
//                     {
//                         println!("call sent fine");
//                     })
//                     .await;
//                 }
//                 Err(e) => {
//                     log::error!("Run update choice system: {e}");
//                 }
//             }
//         }

//     });
// }












#[derive(Resource)]
pub struct CheckGame(mpsc::Sender<()>);

impl CheckGame {
    pub fn try_send(&self)
     -> Result< (), mpsc::error::TrySendError<()> > {
        self.0.try_send(())
    }
}



//some sort of retunr value from dojo?
pub struct GameUpdate {
    pub game_update: Vec<FieldElement>,// vector of field elements
}


// this one whould work, this just fetches the game component to read who the winner is
fn fetch_game_component(
    env: Res<DojoEnv>,
    runtime: ResMut<TokioTasksRuntime>,
    mut commands: Commands,
) {

    let (tx, mut rx) = mpsc::channel::<()>(16);

    commands.insert_resource(CheckGame(tx));

    let account = env.account.clone();
    let world_address = env.world_address;
    let block_id = env.block_id;

    // let player_num_one: FieldElement = PLAYER_NUM_ONE.into();
    //let player_num_two: FieldElement = PLAYER_NUM_TWO.into();

    runtime.spawn_background_task(move |mut ctx| async move {

        let world = WorldContract::new(world_address, account.as_ref());

        let player_component = world.component("Game", block_id).await.unwrap();

        while let Some(_) = rx.recv().await {
           
                match player_component
                    .entity(FieldElement::ZERO, vec![into_field_element(10)], block_id)
                    .await
                {   
                    Ok(update) => 
                    {
                        ctx.run_on_main_thread(move |ctx| 
                        {   
                            let mut state: SystemState<EventWriter<GameUpdate>> = SystemState::new(ctx.world);
                          
                            let mut update_game: EventWriter<'_, GameUpdate> = state.get_mut(ctx.world);
              
                            update_game.send(GameUpdate { game_update: update })  

                        })
                        .await;
                    }

                    Err(e) => {
                        log::error!("Query `Game` component: {e}");
                    }
                }
        }
    });
}



// this is the function that reads the event from above
fn check_game_update(
    mut events: EventReader<GameUpdate>,  //gets an event call
    mut query: Query<&mut Game>,
) {
    for e in events.iter() { //loop through every event
        if let Ok(mut state) = query.get_single_mut() 
        {  
            let onchain_game_state:u32 = field_element_to_u32(e.game_update[0]);

            state.current_game_state = onchain_game_state as u8;

            println!("onchain game state: {}", state.current_game_state)
        }
    }
}













#[derive(Resource)]
pub struct UpdateGameWinnerState(mpsc::Sender<()>);

impl UpdateGameWinnerState {
    pub fn try_send(&self)
        // Result<T, E>: The Result type in Rust represents either a successful value of type T or an error of type E.
        // this is not a touple
     -> Result< (), mpsc::error::TrySendError<()> > {
        // i think its like if it does fail it saves the error in e

        self.0.try_send(())
    }
}



// this runs the funciton to set the winner so the new game state
fn set_winner_state_thread(
    env: Res<DojoEnv>,
    runtime: ResMut<TokioTasksRuntime>,
    mut commands: Commands,
    
) {
    let (tx, mut rx) = mpsc::channel::<()>(8);

    commands.insert_resource(UpdateGameWinnerState(tx));
  
    let account = env.account.clone();
    let world_address = env.world_address;
    let block_id = env.block_id;

    runtime.spawn_background_task(move |mut ctx| async move {

        let world = WorldContract::new(world_address, account.as_ref());

        let check_choice_system = world.system("check_game_dojo_side", block_id).await.unwrap();
        

        while let Some(_) = rx.recv().await {

            match check_choice_system
                .execute(vec![
                    into_field_element(10),
                ])
                .await
            {
                Ok(_) => {
                    ctx.run_on_main_thread(move |_ctx| 
                    {
                        println!("call sent fine");
                        
                    })
                    .await;
                }
                Err(e) => {
                    log::error!("Run check_game_dojo_side system: {e}");
                }
            }
        }

    });
}





























pub fn field_element_to_f32(val: FieldElement) -> f32 {
    val.to_string().parse().unwrap()
}


pub fn field_element_to_u32(val: FieldElement) -> u32 {
    field_element_to_f32(val) as u32
}



pub fn into_field_element(val: u8) -> FieldElement {
    FieldElement::from(val)
}

