# Cellular Automata Simulation

This repository contains a simple 2D cellular automata simulation built using the [Bevy game engine](https://bevyengine.org/). The simulation generates rows of cells based on a set of predefined rules and visualizes them using wireframe graphics.

## Features

- **2D Cellular Automata**: Implements a cellular automaton based on the `NUMBER` constant, which determines the rules for generating the next row of cells.
- **Customizable Grid**: The grid size and cell dimensions are adjustable through constants in the code.
- **Graphical Display**: Renders the simulation using Bevy's sprite and mesh systems, with cells dynamically added based on the automaton's logic.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Bevy](https://bevyengine.org/) (latest version)

### Running the Simulation

1. Clone the repository:

   ```bash
   git clone https://github.com/khalid-khalifah/2d-cellular-automata
   cd your-repo-name
   ```

2. Run the project using cargo
   ```bash
   cargo run
   ```

> it will take some time in the first build.

### Controls

- The simulation runs automatically upon startup, generating new rows based on the rule specified by the `NUMBER` constant in `main.rs`.

### Code Overview

- `ActiveRow`: Manages the current row of the automaton and initializes the first row.
- `CellularAutomataPlugin`: Adds the necessary systems to the Bevy app for simulating the automaton.
- `setup()`: Initializes the camera and draws the initial row of cells.
- `add_row()`: Computes the next row of cells based on the automaton's rules and adds it to the screen.
- `compute_new_row()`: Determines the new state of the cells by applying a rule encoded in the NUMBER constant.

### Customization

- **Grid Size**: The grid size can be adjusted by changing the `SCREEN_RESOLUTION` and `SQUER_SIZE` constants.
- **Automaton Rule**: The rule for generating the automaton can be modified by changing the value of the `NUMBER` constant (represented in binary), check [Wolfram](https://mathworld.wolfram.com/ElementaryCellularAutomaton.html) for the list of the rules
