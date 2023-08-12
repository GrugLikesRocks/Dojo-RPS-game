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





#[derive(Component, Serde, Drop, Copy)]
struct Game {
    winner: u8,
}

impl GameSerdeLen of dojo::SerdeLen<Game> {
    #[inline(always)]
    fn len() -> usize {
        8
    }
}

// 0 nothing yet
// 1 player one wins
// 2 player two wins
// 3 tie


// #[derive(Serde, Drop)]
// enum Choice {
//     Rock: (),     3
//     Paper: (),   2
//     Scissor: (),  1
//     None: (), 0
// }



//so what i think its happening is that its using the model id as the key to get the data, in my case it should be the address of the player or in this case the 
// player field value


// THE SECOND ONE IS ABOUT THIS NONCE VALUE BEING WRONG 



#[system]
mod start_game_dojo_side {
    use array::ArrayTrait;
    use traits::Into;
    use serde::Serde;
    use dojo::world::Context;
    use super::{Player}; //gettign the types from above
    use super:: {Game};
    use super:: {ONE, TWO};


    // this shoud be the starting point and where we spawn the player
    fn execute(ctx: Context, player_index : felt252, address_for_game_bevy : felt252) {
        
        set !(ctx.world, address_for_game_bevy.into(), (Game { winner: 0 }));
        set !(ctx.world, ONE.into(), (Player { index: ONE, choice: ONE }));
        set !(ctx.world, TWO.into(), (Player { index: TWO, choice: TWO }));
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
    use super:: {Game};

    fn execute(ctx: Context, address_for_game_bevy : felt252) {

        let player_one = get !(ctx.world, ONE.into(), Player);   // get the data
        let player_two = get !(ctx.world, TWO.into(), Player);   

        let current_state = get !(ctx.world, address_for_game_bevy.into(), Game);   // get the data


        // assert(player_one.choice == , 'should be 1'); 
        // assert(player_two.choice == 2, 'should be 2'); 

        let mut game_winner: u8 = 0;

        if player_one.choice != 0 && player_two.choice != 0 {
            
            if player_one.choice == 3 && player_two.choice == 1 {
                //player one wins
                game_winner = 1;
            }
            else if player_one.choice == 3 && player_two.choice == 2 {
                //player two wins
                game_winner = 2;    
            }
            else if player_one.choice == 2 && player_two.choice == 3 {
                //player one wins
                game_winner = 1;
            }
            else if player_one.choice == 2 && player_two.choice == 1 {
                //player two wins
                game_winner = 2;
            }
            else if player_one.choice == 1 && player_two.choice == 2 {
                //player one wins
                game_winner = 1;
            }
            else if player_one.choice == 1 && player_two.choice == 3 {
                //player two wins
                game_winner = 2;
            }
            else {
                //tie
                game_winner = 3;
            }
        }


        if current_state.winner != game_winner {
            set !(ctx.world, address_for_game_bevy.into(), (Game { winner: game_winner }));
        }
        
    }
}




// this is to change

#[system]
mod update_player_choice{
    use array::ArrayTrait;
    use traits::Into;
    use serde::Serde;
    use dojo::world::Context;
    use super::{Player}; //gettign the types from above
    use super:: {ONE, TWO};
                                                    // is there a way to send a type? like a enum or something?
    fn execute(ctx: Context, choice_index_one : felt252,  choice_index_two : felt252 )
    {
        
        // assert(player_index_one == 1, 'should be 1'); 
        // assert(player_index_two == 2, 'should be 2'); 

        if choice_index_one == 1 {
            set !(ctx.world, ONE.into(), (Player { index: ONE.into(), choice: 1 }));
        }
        else if choice_index_one == 2 {
            set !(ctx.world, ONE.into(), (Player { index: ONE.into(), choice: 2 }));
        }
        else if choice_index_one == 3 {
            set !(ctx.world, ONE.into(), (Player { index: ONE.into(), choice: 3 }));
        }


        if choice_index_two == 1 {
            set !(ctx.world, TWO.into(), (Player { index: TWO.into(), choice: 1 }));
        }
        else if choice_index_two == 2 {
            set !(ctx.world, TWO.into(), (Player { index: TWO.into(), choice: 2 }));
        }
        else if choice_index_two == 3 {
            set !(ctx.world, TWO.into(), (Player { index: TWO.into(), choice: 3 }));
        }

    }
}