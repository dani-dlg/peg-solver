# Peg solitaire / Senku
A command-line application implementing the game of [Peg Solitaire](https://en.wikipedia.org/wiki/Peg_solitaire)
## Features
- Play the game in the command line.
- Solve the game automatically using a backtracking search or a randomized search. 
- The program allows custom board configurations by changing the `BOARD_REPR` and `BOARD_SIZE` constants.

## Build
To build the program you need the Rust compiler. The easiest to install it is with [rustup](https://www.rust-lang.org/learn/get-started).
Clone or download this repository
```git clone https://github.com/dani-dlg/peg-solver
```
Build the program with
```cd peg-solver
cargo run --release
```

## Usage
Moves are entered with a "coordinate-direction" notation. For example, the move `B3R` means taking the peg at position B3, placing it two holes to the right, and removing the peg that was in the way. 

## To do
This is just a small project to get started with the Rust language. It can be improved in many areas, such as:
- Document/refactor the code
- Parallelize the random search
- Optimization