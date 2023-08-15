
pub mod configuration_values
{
    ///////// DOJO ///////////
    pub const JSON_RPC_ENDPOINT: &str = "http://0.0.0.0:5050";

    //account 0 that deploys the contract
    pub const ACCOUNT_ADDRESS: &str = "0x03ee9e18edc71a6df30ac3aca2e0b02a198fbce19b7480a63a0d71cbd76652e0"; // katana account 0
    pub const ACCOUNT_SECRET_KEY: &str = "0x0300001800000000300000180000000000030000000000003006001800006600";

    pub const ACCOUNT_ADDRESS_PLAYER_ONE: &str = "0x033c627a3e5213790e246a917770ce23d7e562baa5b4d2917c23b1be6d91961c"; // katana account 1
    pub const ACCOUNT_SECRET_KEY_PLAYER_ONE: &str = "0x0333803103001800039980190300d206608b0070db0012135bd1fb5f6282170b";

    pub const ACCOUNT_ADDRESS_PLAYER_TWO: &str = "0x01d98d835e43b032254ffbef0f150c5606fa9c5c9310b1fae370ab956a7919f5";
    pub const ACCOUNT_SECRET_KEY_PLAYER_TWO: &str = "0x07ca856005bee0329def368d34a6711b2d95b09ef9740ebf2c7c7e3b16c1ca9c";

    
    pub const WORLD_ADDRESS: &str = "0x5b328933afdbbfd44901fd69a2764a254edbb6e992ae87cf958c70493f2d201";
    pub const DOJO_SYNC_INTERVAL: f32 = 0.5;
    pub const MODEL_NAME: &str = "model";

    ///////// MICS ///////////
    
    use bevy::prelude::Color;

    pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
    pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
    pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35); 
    
}



