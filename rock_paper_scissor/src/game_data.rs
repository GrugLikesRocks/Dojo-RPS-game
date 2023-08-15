use bevy::prelude::*;


pub enum Outcome {
    Player1Wins = 1,
    Player2Wins = 2,
    Tie = 3,
    None = 4,
}

impl From<u32> for Outcome {
    fn from(item: u32) -> Self {
        match item {
            1 => Outcome::Player1Wins,
            2 => Outcome::Player2Wins,
            3 => Outcome::Tie,
            4 => Outcome::None,
            _ => panic!("Invalid value for Outcome enum!"), // or you might return a default value or Error
        }
    }
}




#[derive(Component)]
pub struct Game {
    pub player1: Entity,
    pub player2: Entity,
    pub outcome: Outcome,
}

#[derive(Component)]
pub struct Player {
    pub choice: Choice,
    pub crypto_address: CryptoAddress, // i would like for this to be type address or something
}

pub struct CryptoAddress {
    pub address: String,
    pub secret: String,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
    None = 4,
}

impl Into<u8> for Choice {
    fn into(self) -> u8 {
        self as u8
    }
}


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ButtonMaterials {
    Normal,
    Hovered,
    Pressed,
}

#[derive(Component)]
pub struct ButtonPlayerData {
    pub player: Entity,
    pub choice: Choice,
}

#[derive(Component)]
pub struct TextPlayerData {
    pub player: Entity,
}

#[derive(Component)]
pub struct TextGameOutcome;
