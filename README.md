# I am learning Rust

Against the Storm is an amazing game that single-handedly defined a new genre of
roguelike citybuilders. I am making a demake of that game, called Rusty Rain.
The name clearly indicates that the goal of this project is for me to learn Rust
(I am a dotnet fullstack developer, so... good luck me). This Crate encompases
the backend for the game. Please check rusty_rain_tui for a frontend
implementation (presumably in ratatui, but I have barely started, so might not
be available yet on github).

Curently the base features of the game are being implemented, and still subject
to change:

- base world representation
- base structures like hearth, store, gathering huts
- base AI and pathfinding

Currently not implemented, but should be the short-term focus:

- generalizing woodcutter for other gathering huts
- production huts
- advanced worker logic - breaks, food and mood
- housing, vanity buildings
- world generation

The longterm tasks, that won't be started soon

- metagame: perks, blueprints, quests, events
- impatience system
- still considering utilizing some simple ECS system... e.g. HECS

The assumptions of this project:

- the game/simulation should be deterministic. I would like to implement
  lockstep multiplayer in the future.
- low performance/memory footprint.
- not 100% faithfull to the original, which still remains a major inspiration
- learning is fun!
- little dependencies

Godspeed!
