use bevy::prelude::*;

// components and enums structs used all over the project usually each type should have its own script but because there are not many this is how we are doing it here

pub enum Outcome {
    Player1Wins = 1,
    Player2Wins = 2,
    Tie = 3,
    None = 4,
}

// impl is an extention to a type
// in this case when we have an u32 we can call .into() on it and it will return an Outcome enum
impl From<u32> for Outcome {
    fn from(item: u32) -> Self {
        match item {
            1 => Outcome::Player1Wins,
            2 => Outcome::Player2Wins,
            3 => Outcome::Tie,
            4 => Outcome::None,
            _ => panic!("Invalid value for Outcome enum!"),
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
    pub crypto_address: CryptoAddress, 
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

// otherway around here we can call .into() on a Choice enum and it will return an u8
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

// some components are completly empty and are used only to identify entities
// so in this case when we qeury the world for entities with this component we know that we are looking for a text that displays the game outcome
#[derive(Component)]
pub struct UIGameOutcome;
