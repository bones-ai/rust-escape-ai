## Escape AI
AI learns escape a room. 

This is a [rust](https://www.rust-lang.org/) based implementation of a genetic algorithm and reinforcement learning simulation that trains an AI named Zoe to escape from five rooms of increasing difficulty. The simulation is built using the [macroquad](https://macroquad.rs/) library.

## Demo
[![youtube](https://img.youtube.com/vi/OeojCLDKaJU/0.jpg)](https://www.youtube.com/watch?v=OeojCLDKaJU)

## Usage
- Clone the repo
    ```
    git clone git@github.com:bones-ai/rust-escape-ai.git
    cd rust-escape-ai
    ```
- Run the simulation
    ``` 
    cargo run --release
    ```
- To update the simulation configurations use the configs file located at `src/configs.rs`

## Create custom rooms
- [Tiled map editor](https://www.mapeditor.org/) is used for building the 2d levels
- To create custom levels and play around with the AI, use tile layers with names `player`, `keys`, `door`, `walls`, `spikes`, `enemies` and `background`. I suggest taking a look at how other maps are built to understand how things are to be setup.

## Configurations
- The project config file is located at `src/configs.rs`
- The game rooms have to be manually configured in the config file.

## Inputs
- `Spacebar` - Pause/Unpause the simulation
- `Tab` - Show/hide the egui control menu
- `r` - Restart the simulation
- `Backspace` - Slow mode
- `Backslash` - Enable AI, start the AI training process (Use this to play the game yourself, using keyboard inputs wasd)
- `Right Shift` - Run at 5x speed
- `Mouse wheel` - Zoom in/out
- `Mouse wheel with ctrl` - Large zoom in/out
- `Right mouse click drag` - Pan through the world
- To draw all training AI agents, use the egui menu, then select `Show Multi`. Caution, if you have too many game rooms being simulation, this could freeze your machine.

## Assets
- [https://fisherg.itch.io/micro-asset-pack](https://fisherg.itch.io/micro-asset-pack)

## If Zoe isn't able to solve a room
- Try restarting (using shortcut `r`). It's impossible to get the same results as in the youtube video as every simulation run is random. Sometimes Zoe might get stuck (or spend a lot of time/steps) in a region, letting the simulation run for a longer is an option (but it'll take a long time).
- Update `NUM_FRAMES` to a larger value. Once the room is solved, Zoe will try to solve the room more efficiently (i.e taking less steps)
- Run the simulation at 5x (by using shortcut `Right Shift`). Some rooms can take a lot of generations to solve.
