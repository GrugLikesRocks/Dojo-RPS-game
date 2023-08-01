use starknet::ContractAddress;

#[derive(Component, Copy, Drop, Serde, SerdeLen)]
struct Game {
    game_id: u32,
    start_time: u64,
    end_time: u64,
    max_players: usize,
    max_turns: usize,
    is_finished: bool,
    creatorId: ContractAddress,
    winner: ContractAddress,
}

#[derive(Component, Copy, Drop, Serde, SerdeLen)]
struct PlayersId {
    player_one: ContractAddress,
    player_two: ContractAddress,
}

#[derive(Component, Copy, Drop, Serde, SerdeLen)]
struct GameTurn {
    turn: PlayersId, 
}

trait GameTrait {
    fn tick(self: @Game) -> bool;
}

impl GameImpl of GameTrait {
    fn tick(self: @Game) -> bool {
        let info = starknet::get_block_info().unbox();

        if info.block_timestamp < *self.start_time {
            return false;
        }
        if *self.is_finished {
            return false;
        }

        true
    }
}