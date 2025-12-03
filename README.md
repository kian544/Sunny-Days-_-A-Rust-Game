
# Sunny Days - A Rust Game

To run the game, first clone this repo to your local machine, then enter the terminal on the "Sunny Days" Folder, and use the cmd:
cargo clean - To clean out any previous runs
cargo run - To begin running the game

Once in the game, simply click Q, or q, to quit the game, you may move around by using the WASD/Key Arrows

https://youtu.be/5j8qE_WO06I?si=JBefL3HPSe7lxzlX (Youtube link to explanation and demo)

DOCUMENTATION:

The Project Overview Sunny-Days is a narrative driven adventure where the player explores procedurally generated maps, manages an inventory, fights turn based battles, and uncovers a conspiracy involving the town Mayor. It runs natively in the terminal using crossterm for raw input handling and ratatui for rendering the interface.

Working on this project really solidified my understanding of Rust's Ownership and Borrowing model. Coming from other languages, I was used to just passing pointers around freely. Rust forced me to think about the lifetime of my game data.

I learned that a Game Loop is actually one of the hardest things to implement in Rust because you often want to mutate the player (update HP) and the enemy (deal damage) simultaneously, which the borrow checker hates. I also learned how powerful Rust's Enums and Pattern Matching are. Using Option<T> for inventory slots and the GameState enum made the logic incredibly type safe, I rarely had runtime crashes because the compiler caught logic errors early.

Using the ratatui crate made creating the dashboard (sidebar, logs, map view) very intuitive. The layout system uses constraints that automatically resize based on the terminal window, which gave me a responsive UI 'for free.'
The procedural generation using rand was straightforward logic—carving out rectangles and connecting them. Rust's vector manipulation made grid management clean.

The hardest part, by far, was the Battle System Architecture. In world.rs, I have a World struct that owns everything, the player, the enemies, and the logs. During a battle, I need to read the Enemy's stats (an immutable borrow of World) and then update the Player's HP or the Action Log (a mutable borrow of World). Rust explicitly forbids having a mutable borrow while an immutable borrow is active. I spent a lot of time fighting the compiler here, getting errors like cannot borrow *self as mutable more than once at a time

1. Architecture Overview
The system follows a modified Entity Component System (ECS) pattern where the World struct acts as the central source of truth.
Input Layer (tui/input.rs): Captures raw key events from the OS.
Game Loop (engine/game_loop.rs): A fixed time-step loop (60ms tick rate). It polls for input, maps keys to abstract Action enums, and triggers the render cycle.
Logic Layer (engine/world.rs): Processes Actions, updates game state, and handles combat/dialogue logic.
Presentation Layer (tui/renderer.rs): Draws the state to the terminal buffer

2. Core Modules
A. The Engine (src/engine/)
world.rs: The "Brain." It holds the levels, player, npcs, and logs. It implements the apply_action method which is the primary state transition function. It manages the flags for the narrative (e.g., mayor_defeated, dorosht_completed).
entity.rs: Defines data structures for Player, Inventory, Equipment, and Consumable. It includes logic for stat calculation (base stats + equipment bonuses + temporary buffs).
action.rs: An Enum defining every possible player intent (Move, Interact, BattleOption, ToggleInventory), decoupling input keys from game logic.

B. Map System (src/map/)
generator.rs: Uses a BSP like (Binary Space Partitioning) approach to place non-overlapping rooms and connect them with L-shaped corridors.
tile.rs: Defines Tile types (Wall, Floor, Door, Chest). The door logic was recently updated to act as a solid obstacle that requires interaction, rather than a walkable tile.

C. UI & Rendering (src/tui/)
renderer.rs: Handles the "Camera." It calculates a viewport offset (compute_viewport_origin) to keep the player centered. It conditionally renders widgets based on GameState (e.g., swapping the Log widget for a Battle Menu widget when combat starts).

3. Key Algorithms & Mechanics

Turn-Based Combat:
Initiative: Determined by the speed stat.
Damage: (Attack * 1.2).
Deflection: (Defense / 10) * 0.2 probability to negate damage entirely.
Penalty Timer: The game loop tracks last_battle_input. If >10 seconds pass without input, a penalty flag is passed to the engine, forcing the enemy to attack first regardless of speed.

State Machine:
The GameState enum controls the flow: Title -> Intro -> Playing <-> Dialogue / Battle -> Fin.
This prevents invalid actions (e.g., you cannot move while in a dialogue tree).

Inventory System:
Implemented as a struct with Vec<Consumable> and Vec<Equipment>.
Supports equipping/unequipping items which dynamically modify the player's max_hp, atk, def, and spd.


BATTLE SYSTEM RULES:

Battles should remain in the log, similar to a dialogue box with a character or NPC

When a battle is initiated, there should be 3 options, 1 is Fight, 2 is Inventory (Allows the use of one turn to use a consumable), 3 is to Run (Flees the battle, random percentage chance to flee)

Whatever weapons are equipped prior to the battle cannot be changed once in battle

Who ever has a higher total speed, can be including a temporary buff, attacks first.

Each attack does 1.2x damage of the total ATK stat, IE if the player has 13 attack, one attack should do 15.6 damage, etc. 

The defense stat gives a chance at deflecting an attack, the higher the DEF stat, the higher the chances, 20% per 10 DEF, so if someone has 40 DEF, they have an 80% chance at deflecting any attack, even multiple times in a row, each attack is an independent event. 

If the player attacks the enemy first, fleeing is not an option, “You started this, finish it!”

After each set of attacks, IE user attacks, then enemy attacks, the battle goes back to initial battle state, IE 1 attack, 2 Inventory

If they player takes longer than 10 seconds to make a move, the enemy will attack first for the rest of the battle, regardless of speed. 

Whoever reaches 0 health first loses, the opponent health bar is visible above the dialog box for options 1, 2, and 3. 

If the player dies, the entire game is over, and exits. 
