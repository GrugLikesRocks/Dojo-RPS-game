use traits::Into;
use cubit::types::vec2::{Vec2, Vec2Trait};
use cubit::types::fixed::{Fixed, FixedTrait, ONE_u128};
use cubit::math::{trig, comp::{min, max}, core::{pow_int, sqrt}};
use starknet::ContractAddress; //used
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
    player_ca: felt252, //address of the driver contract
    choice: felt252, // this should be an enum but whatever
}

//seerde is something to do with the indexer but i dont know what it does
impl PlayerSerdeLen of dojo::SerdeLen<Player> {
    #[inline(always)]
    fn len() -> usize {
        8
    }
}


// 1 r
// 2 p
// 3 s
// 4 nothing yet

#[derive(Component, Serde, Drop, Copy)]
struct Game {
    winner: u8,
    game_ca: felt252, //address of the driver contract
    player_one_address: felt252,
    player_two_address: felt252,
}

impl GameSerdeLen of dojo::SerdeLen<Game> {
    #[inline(always)]
    fn len() -> usize {
        8
    }
}


// 1 player one wins
// 2 player two wins
// 3 tie
// 4 nothing yet

// this starts the game by taking the address and where to store everything

#[system]
mod start_game_dojo_side {
    use array::ArrayTrait;
    use traits::Into;
    use serde::Serde;
    use dojo::world::Context;
    use super::{Player};
    use super::{Game};
    use starknet::ContractAddress;

    fn execute(
        ctx: Context,
        player_one_address: felt252,
        player_two_address: felt252,
        game_address: felt252
    ) {
        set !(
            ctx.world,
            player_one_address.into(),
            (Player { player_ca: player_one_address, choice: 4 })
        );
        set !(
            ctx.world,
            player_two_address.into(),
            (Player { player_ca: player_one_address, choice: 4 })
        );
        set !(
            ctx.world,
            game_address.into(),
            (Game {
                winner: 0,
                game_ca: game_address,
                player_one_address: player_one_address,
                player_two_address: player_two_address
            })
        );
    }
}


// this checks and returns the current state of the game by checking what both of the players have chosen

#[system]
mod check_game_dojo_side {
    use array::ArrayTrait;
    use traits::Into;
    use serde::Serde;
    use dojo::world::Context;
    use super::{Player}; //gettign the types from above
    use super::{Game};

    fn execute(ctx: Context, address_for_game_bevy: felt252) {
        let game = get !(ctx.world, address_for_game_bevy.into(), Game); // get the data

        let player_one = get !(ctx.world, game.player_one_address.into(), Player); // get the data
        let player_two = get !(ctx.world, game.player_two_address.into(), Player);

        let mut game_winner: u8 = 0;

        if player_one.choice != 4 && player_two.choice != 4 {
            if player_one.choice == 3 && player_two.choice == 1 {
                //player one wins
                game_winner = 1;
            } else if player_one.choice == 3 && player_two.choice == 2 {
                //player two wins
                game_winner = 2;
            } else if player_one.choice == 2 && player_two.choice == 3 {
                //player one wins
                game_winner = 1;
            } else if player_one.choice == 2 && player_two.choice == 1 {
                //player two wins
                game_winner = 2;
            } else if player_one.choice == 1 && player_two.choice == 2 {
                //player one wins
                game_winner = 1;
            } else if player_one.choice == 1 && player_two.choice == 3 {
                //player two wins
                game_winner = 2;
            } else {
                //tie
                game_winner = 3;
            }
        } else {
            //nothing yet
            game_winner = 4;
        }

        if game.winner != game_winner {
            set !(
                ctx.world,
                address_for_game_bevy.into(),
                (Game {
                    winner: game_winner.into(),
                    game_ca: address_for_game_bevy,
                    player_one_address: game.player_one_address,
                    player_two_address: game.player_two_address
                })
            );
        }
    }
}


// this sets what the specific player has chosen

#[system]
mod update_player_choice {
    use array::ArrayTrait;
    use traits::Into;
    use serde::Serde;
    use dojo::world::Context;
    use super::{Player}; //gettign the types from above
    use super::{ONE, TWO};
    // is there a way to send a type? like a enum or something?
    fn execute(ctx: Context, player_address: felt252, choice: felt252) {
        set !(
            ctx.world, player_address.into(), (Player { player_ca: player_address, choice: choice })
        );
    // if choice_index_one == 1 {
    //     set !(ctx.world, ONE.into(), (Player { index: ONE.into(), choice: 1 }));
    // }
    // else if choice_index_one == 2 {
    //     set !(ctx.world, ONE.into(), (Player { index: ONE.into(), choice: 2 }));
    // }
    // else if choice_index_one == 3 {
    //     set !(ctx.world, ONE.into(), (Player { index: ONE.into(), choice: 3 }));
    // }

    // if choice_index_two == 1 {
    //     set !(ctx.world, TWO.into(), (Player { index: TWO.into(), choice: 1 }));
    // }
    // else if choice_index_two == 2 {
    //     set !(ctx.world, TWO.into(), (Player { index: TWO.into(), choice: 2 }));
    // }
    // else if choice_index_two == 3 {
    //     set !(ctx.world, TWO.into(), (Player { index: TWO.into(), choice: 3 }));
    // }
    }
}

