# Dojo-RPS-game
A classic Rock, Paper, Scissors game written with Dojo engine

# Commands to run
cd rock_paper_scissor

# Start Katana
katana --seed 0 --block-time 1

# Build the game
sozo build

# Migrate the world, this will declare/deploy contracts to katana
sozo migrate 

# Start indexer, graphql endpoint at http://localhost:8080
torii --manifest target/dev/manifest.json --world-address 0x5b328933afdbbfd44901fd69a2764a254edbb6e992ae87cf958c70493f2d201

# start game
cargo run

