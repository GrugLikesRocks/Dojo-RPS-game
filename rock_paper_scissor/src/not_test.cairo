use traits::Into;
use cubit::types::vec2::{Vec2, Vec2Trait};
use cubit::types::fixed::{Fixed, FixedTrait, ONE_u128};
use cubit::math::{trig, comp::{min, max}, core::{pow_int, sqrt}};
use starknet::ContractAddress;    //used
use array::{ArrayTrait, SpanTrait};
use serde::Serde;
use orion::operators::tensor::core::{Tensor, TensorTrait, ExtraParams};
use orion::numbers::fixed_point::core as orion_fp;
use orion::numbers::fixed_point::implementations::impl_16x16::FP16x16Impl;
use orion::operators::tensor::implementations::impl_tensor_fp::Tensor_fp;



const ONE: felt252 = 1;
const TWO: felt252 = 2;


#[derive(Component, Serde, Drop, Copy)]
struct Player {
    index: felt252,    //idk the int type for cairo so we are going to use enums for now, or not lol
    choice: felt252, 
    //driver: ContractAddress, //address of the driver contract
}

//seerde is something to do wiht the indexer but i dont know what it does
impl PlayerSerdeLen of dojo::SerdeLen<Player> {
    #[inline(always)]
    fn len() -> usize {
        8
    }
}


// #[derive(Serde, Drop)]
// enum Choice {
//     Rock: (),     3
//     Paper: (),   2
//     Scissor: (),  1
//     None: (), 0
// }



//so what i think its happening is that its using the model id as the key to get the data, in my case it should be the address of the player or in this case the 
// player field value



#[system]
mod start_game_dojo_side {
    use array::ArrayTrait;
    use traits::Into;
    use serde::Serde;
    use dojo::world::Context;
    use super::{Player}; //gettign the types from above

    // this shoud be the starting point and where we spawn the player
    fn execute(ctx: Context, player_index : felt252) {
        

        // two sets, set! is what i think should be the thing that spawns or "sets" data in the world
        // the arguments are     the rworld context     a key and a value    very similar to a normal dict
        // the key in this case cna be the adress right?
        // key being the address should point to the right player struct

        let start_choice : felt252 = 0;

        set !(ctx.world, player_index.into(), (Player { index: player_index, choice: start_choice }));
    }
}

#[system]
mod check_game_dojo_side{
    use array::ArrayTrait;
    use traits::Into;
    use serde::Serde;
    use dojo::world::Context;
    use super::{Player}; //gettign the types from above
    use super:: {ONE, TWO};

    fn execute(ctx: Context) {

        let player_one = get !(ctx.world, ONE.into(), Player);   // get the data
        let player_two = get !(ctx.world, TWO.into(), Player);   

        if player_one.choice != 0 && player_two.choice != 0 {
            
            if player_one.choice == 3 && player_two.choice == 1 {
                //player one wins
            }
            else if player_one.choice == 3 && player_two.choice == 2 {
                //player two wins
            }
            else if player_one.choice == 2 && player_two.choice == 3 {
                //player one wins
            }
            else if player_one.choice == 2 && player_two.choice == 1 {
                //player two wins
            }
            else if player_one.choice == 1 && player_two.choice == 2 {
                //player one wins
            }
            else if player_one.choice == 1 && player_two.choice == 3 {
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
    use super::{Player}; //gettign the types from above
    use super:: {ONE, TWO};
                                                    // is there a way to send a type? like a enum or something?
    fn execute(ctx: Context, player_index : felt252, choice_index : felt252) {
        
        if choice_index == 1 {
            set !(ctx.world, player_index.into(), (Player { index: player_index, choice: 3 }));
        }
        else if choice_index == 2 {
            set !(ctx.world, player_index.into(), (Player { index: player_index, choice: 2 }));
        }
        else if choice_index == 3 {
            set !(ctx.world, player_index.into(), (Player { index: player_index, choice: 1 }));
        }
    }
}