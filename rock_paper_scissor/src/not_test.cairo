use traits::Into;
use cubit::types::vec2::{Vec2, Vec2Trait};
use cubit::types::fixed::{Fixed, FixedTrait, ONE_u128};
use cubit::math::{trig, comp::{min, max}, core::{pow_int, sqrt}};
use starknet::ContractAddress;    //used
use drive_ai::{Vehicle, VehicleTrait};
use drive_ai::enemy::{Position, PositionTrait};
use drive_ai::math::{intersects};
use drive_ai::rays::{RaysTrait, Rays, Ray, RayTrait, NUM_RAYS, RAY_LENGTH};
use array::{ArrayTrait, SpanTrait};

use orion::operators::tensor::core::{Tensor, TensorTrait, ExtraParams};
use orion::numbers::fixed_point::core as orion_fp;
use orion::numbers::fixed_point::implementations::impl_16x16::FP16x16Impl;
use orion::operators::tensor::implementations::impl_tensor_fp::Tensor_fp;





const ONE: felt252 = 1;
const TWO: felt252 = 2;


#[derive(Component, Serde, Drop, Copy)]
struct Player {
    index: felt252,    //idk the int type for cairo so we are going to use enums for now
    choice: Choice, 
    //driver: ContractAddress, //address of the driver contract
}


#[derive(Serde, Drop)]
enum Choice {
    Rock: (),
    Paper: (),
    Scissor: (),
    None: (),
}



//so what i think its happening is that its using the model id as the key to get the data, in my case it should be the address of the player or in this case the 
// player field value



#[system]
mod start_game_dojo_side {
    use array::ArrayTrait;
    use traits::Into;
    use serde::Serde;
    use dojo::world::Context;
    use super::{Player, PlayerNum, Choice}; //gettign the types from above

    // this shoud be the starting point and where we spawn the player
    fn execute(ctx: Context, player_index : felt252) {
        

        // two sets, set! is what i think should be the thing that spawns or "sets" data in the world
        // the arguments are     the rworld context     a key and a value    very similar to a normal dict
        // the key in this case cna be the adress right?
        // key being the address should point to the right player struct

        set !(ctx.world, player_index, (Player { index: player_index, choice: Choice::None }));
    }
}

#[system]
mod check_game_dojo_side{
    use array::ArrayTrait;
    use traits::Into;
    use serde::Serde;
    use dojo::world::Context;
    use super::{Player, PlayerNum, Choice}; //gettign the types from above

    fn execute(ctx: Context) {

        let player_one = get !(ctx.world, ONE, Player);   // get the data
        let player_two = get !(ctx.world, TWO, Player);   

        if player_one.choice != Choice::None && player_two.choice != Choice::None {
            
            if player_one.choice == Choice::Rock && player_two.choice == Choice::Scissor {
                //player one wins
            }
            else if player_one.choice == Choice::Rock && player_two.choice == Choice::Paper {
                //player two wins
            }
            else if player_one.choice == Choice::Paper && player_two.choice == Choice::Rock {
                //player one wins
            }
            else if player_one.choice == Choice::Paper && player_two.choice == Choice::Scissor {
                //player two wins
            }
            else if player_one.choice == Choice::Scissor && player_two.choice == Choice::Paper {
                //player one wins
            }
            else if player_one.choice == Choice::Scissor && player_two.choice == Choice::Rock {
                //player two wins
            }
            else {
                //tie
            }
        }
    }
}

#[system]
mod set_player_choice{
    use array::ArrayTrait;
    use traits::Into;
    use serde::Serde;
    use dojo::world::Context;
    use super::{Player, PlayerNum, Choice}; //gettign the types from above

                                                    // is there a way to send a type? like a enum or something?
    fn execute(ctx: Context, player_index : felt252, choice_index : felt252) {
        
        if choice_index == 1 {
            set !(ctx.world, player_index, (Player { index: player_index, choice: Choice::Rock }));
        }
        else if choice_index == 2 {
            set !(ctx.world, player_index, (Player { index: player_index, choice: Choice::Paper }));
        }
        else if choice_index == 3 {
            set !(ctx.world, player_index, (Player { index: player_index, choice: Choice::Scissor }));
        }
    }
}