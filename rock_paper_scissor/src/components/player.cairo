#[derive(Component, Copy, Drop, Serde, SerdeLen)]
struct Player {
    points: u8,
    turns_remaining: usize,
}

trait PlayerTrait {
    fn is_winner(self: @Player) -> bool;
    fn can_continue(self: @Player) -> bool;
}

impl PlayerImpl of PlayerTrait {
    fn is_winner(self: @Player) -> bool {
        //let's set winner to whoever gets to 3 points
        if *self.points == 3 {
            return true;
        }

        false
    }
    fn can_continue(self: @Player) -> bool {
        if *self.turns_remaining == 0 {
            return false;
        }

        true
    }
}