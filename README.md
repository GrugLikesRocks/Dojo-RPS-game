# Dojo-RPS-game
A classic Rock, Paper, Scissors game written with Dojo engine

# Commands to run
cd rock_paper_scissor

# Start Katana
katana --seed 0 --block-time 1

# Build the game
sozo build

# Migrate the world, this will declare/deploy contracts to katana
sozo migrate --name rps     if it throws problems just do it without the --name bit

# Start indexer, graphql endpoint at http://localhost:8080
torii --manifest target/dev/manifest.json --world-address 0x26065106fa319c3981618e7567480a50132f23932226a51c219ffb8e47daa84

# start game
cargo run

