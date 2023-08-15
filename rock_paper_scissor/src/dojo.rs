
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::utils::tracing::field;
use bevy::window::PrimaryWindow;

use crate::configs::configuration_values;
use crate::dojo_utils::*;
use crate::game_data::*;

use bevy::ecs::system::SystemState;
use bevy::log;
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


#[derive(Component)]
pub struct GameData 
{
    started : bool,
}



#[derive(Resource)]
pub struct DojoEnv {
    /// The block ID to use for all contract calls.
    block_id: BlockId,
    /// The address of the world contract.
    world_address: FieldElement,
    /// The account to use for performing execution on the World contract.
    account: Arc<SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>>,
}


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




//////////////////////////////////////////////////////////////////////////////////////////////////////////////


pub struct DojoPlugin;

impl Plugin for DojoPlugin {
    fn build(&self, app: &mut App) {
        let url = Url::parse(configuration_values::JSON_RPC_ENDPOINT).unwrap();
        let account_address = FieldElement::from_str(configuration_values::ACCOUNT_ADDRESS).unwrap();
        let account = SingleOwnerAccount::new(
            JsonRpcClient::new(HttpTransport::new(url)),
            LocalWallet::from_signing_key(SigningKey::from_secret_scalar(
                FieldElement::from_str(configuration_values::ACCOUNT_SECRET_KEY).unwrap(),
            )),
            account_address,
            cairo_short_string_to_felt("KATANA").unwrap(),
        );

        let world_address = FieldElement::from_str(configuration_values::WORLD_ADDRESS).unwrap();

        app.add_plugin(TokioTasksPlugin::default())
            .add_event::<GameUpdate>()
            .add_event::<CheckGame>()

            .insert_resource(DojoEnv::new(world_address, account))

            .add_startup_systems((
                setup_dojo,
                spawn_players_dojo,
                fetch_game_component,
                update_choices_thread, 
               set_winner_state_thread,
            ))
            .add_system(sync_dojo_state)
            .add_system(check_game_update)
            ;
    }
}







fn setup_dojo(mut commands: Commands) {
    commands.spawn(DojoSyncTime::from_seconds(configuration_values::DOJO_SYNC_INTERVAL));
    commands.spawn(GameData { started: false });
}





////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////



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


fn sync_dojo_state(
    //players_query: Query<(&Player,)>,
    mut dojo_sync_time: Query<&mut DojoSyncTime>,
    mut game: Query<&mut GameData>,
    time: Res<Time>,
    spawn_players: Res<StartGameCommand>,   
    change_game_state : Res<UpdateGameWinnerState>
    //check_game_component: Res<CheckGame>,
) {
    let mut dojo_time = dojo_sync_time.single_mut();
 

    let mut game_state = game.single_mut();
    if dojo_time.timer.just_finished() {
        dojo_time.timer.reset();
        
        if game_state.started == false {

            if let Err(e) = spawn_players.try_send() {  
                log::error!("Spawn players channel: {e}");
            }

            game_state.started = true;
        } 
        else { 
          
            if let Err(e) = change_game_state.try_send() {   // thsi one also works fine
                log::error!("updating the game state: {e}");
            }
        }   
    } 
    else {
        dojo_time.timer.tick(time.delta()); 
    }
}



////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////




#[derive(Resource)]
pub struct StartGameCommand(mpsc::Sender<()>);

impl StartGameCommand {
    pub fn try_send(&self)
     -> Result< (), mpsc::error::TrySendError<()> > {
        self.0.try_send(())
    }
}

// this spawns the players
fn spawn_players_dojo(
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

    let player_one_address = FieldElement::from_str(configuration_values::ACCOUNT_ADDRESS_PLAYER_ONE).unwrap();
    let player_two_address = FieldElement::from_str(configuration_values::ACCOUNT_ADDRESS_PLAYER_TWO).unwrap();
    let game_address = FieldElement::from_str(configuration_values::ACCOUNT_ADDRESS).unwrap();
    
    runtime.spawn_background_task(move |mut ctx| async move {

        let world = WorldContract::new(world_address, account.as_ref());

        let start_game_system = world.system("start_game_dojo_side", block_id).await.unwrap();
        while let Some(_) = rx.recv().await {
            match start_game_system
                .execute(vec![
                    player_one_address,
                    player_two_address,
                    game_address
                ])
                .await
               
            {
                Ok(_) => {
                    ctx.run_on_main_thread(move |ctx| 
                    {
                        // let mut state: SystemState<EventWriter<PlayerInitiatedEvent>> = SystemState::new(ctx.world);
                             
                        // let mut spawn_player= state.get_mut(ctx.world);
                        
                        // spawn_player.send(PlayerInitiatedEvent); 


                        println!("spwanw the player fine");
                        
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




////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////



 #[derive(Debug, Clone)]
pub struct PlayerChoiceData {
   pub player_address: FieldElement,
   pub player_choice: FieldElement,
}

#[derive(Resource)]
pub struct UpdateChoices(mpsc::Sender<PlayerChoiceData>);

impl UpdateChoices {
    pub fn try_send(&self, player_address: FieldElement, player_choice: FieldElement)
     -> Result<(), mpsc::error::TrySendError<PlayerChoiceData>> {
        self.0.try_send(PlayerChoiceData {
            player_address,
            player_choice,
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
                    data.player_address,
                    data.player_choice,
                ])
                .await
            {
                Ok(_) => {
                    ctx.run_on_main_thread(move |_ctx| {
                        println!("updating the choices of the player fine");
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











// ////////////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////////////




// this should be called on the button call
#[derive(Resource)]
pub struct CheckGame(mpsc::Sender<()>);

impl CheckGame {
    pub fn try_send(&self)
     -> Result< (), mpsc::error::TrySendError<()> > {
        self.0.try_send(())
    }
}

pub struct GameUpdate {
    pub game_update: Vec<FieldElement>,// vector of field elements
}


// this one should work, this just fetches the game component to read who the winner is
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

    let game_address = FieldElement::from_str(configuration_values::ACCOUNT_ADDRESS).unwrap();

    runtime.spawn_background_task(move |mut ctx| async move {

        let world = WorldContract::new(world_address, account.as_ref());

        let player_component = world.component("Game", block_id).await.unwrap();

        while let Some(_) = rx.recv().await {
           
                match player_component
                    .entity(FieldElement::ZERO, vec![game_address], block_id)
                    .await
                {   
                    Ok(update) => 
                    {
                        ctx.run_on_main_thread(move |ctx| 
                        {   
                            println!("getting the component game fine");
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
            let onchain_game_state:u32 = felt_to_u32(e.game_update[0]);
            println!("onchain_game_state: {:?}", onchain_game_state);
            state.outcome = onchain_game_state.into();  // this is an issue
        }
    }
}




// ////////////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////////////











#[derive(Resource)]
pub struct UpdateGameWinnerState(mpsc::Sender<()>);

impl UpdateGameWinnerState {
    pub fn try_send(&self)
     -> Result< (), mpsc::error::TrySendError<()> > {
        self.0.try_send(())
    }
}



// this runs the funciton to set the winner so the new game state so its just an update
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

    let game_address = FieldElement::from_str(configuration_values::ACCOUNT_ADDRESS).unwrap();

    runtime.spawn_background_task(move |mut ctx| async move {

        let world = WorldContract::new(world_address, account.as_ref());

        let check_choice_system = world.system("check_game_dojo_side", block_id).await.unwrap();
        
        while let Some(_) = rx.recv().await {

            match check_choice_system
                .execute(vec![
                    game_address//into_field_element(10), // this si the index of the game 
                ])
                .await
            {
                Ok(_) => {
                    ctx.run_on_main_thread(move |_ctx| 
                    {
                        println!("setting the winner state fine");
                        
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










