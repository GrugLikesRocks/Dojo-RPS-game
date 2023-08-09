use array::{ArrayTrait, SpanTrait};
//use option::OptionTrait;
//use traits::{Into, TryInto};

#[derive(Component, Copy, Drop, Serde, SerdeLen)]
struct Move {
    name: felt252
}

trait MoveTrait {
   fn all() -> Span<felt252>; 
}

impl MoveImpl of MoveTrait{
    fn all() -> Span<felt252> {
        let mut moves = array::ArrayTrait::new();
        moves.append('Rock'.into());
        moves.append('Paper'.into());
        moves.append('Scissors'.into());

        move.span()
    }   
}