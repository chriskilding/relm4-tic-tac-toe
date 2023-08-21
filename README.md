# relm4-tic-tac-toe

Relm4 tic-tac-toe example

## Overview

This project demonstrates state management in [Relm4](https://github.com/Relm4/Relm4), using a simplified tic-tac-toe game.

In addition to playing the game, the main state management operations are:

- Starting a new game (clearing the board)
- Saving a game (to a file)
- Loading a game (from a file)

## Aims

The project initially aims to explore the Relm4 state management APIs *as they currently exist* in v0.6, including any bugs that may be present.

The code is intentionally simplified to emphasise the main concepts. It omits some common extras (such as `libadwaita` or i18n), which you would use in a production application to improve the user experience, but which would add complexity to the example code.

## Usage

Compile the application:

```shell
cargo build
```

Then run it:

```shell
cargo run
```
