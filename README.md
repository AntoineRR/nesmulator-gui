# Nesmulator GUI

This project is an example of how to handle a GUI with the (nesmulator-core)[https://github.com/AntoineRR/nesmulator-core] crate.

## Current features

* [X] Display the game screen
* [X] A debugging view (display of pattern tables and palette) can be toggled
* [X] First Controller mapping for keyboard
* [X] CLI with various flags

The GUI is created using [winit](https://github.com/rust-windowing/winit) and [pixels](https://github.com/parasyte/pixels).
The sound is handled by [sdl2](https://github.com/Rust-SDL2/rust-sdl2).

## How to run

You will need Rust >= 1.56.0 (2021 edition).
Run the following commands in a terminal:

```
$ git clone https://github.com/AntoineRR/nesmulator-gui
$ cd nesmulator-gui
$ cargo run --release -- <OPTIONS> <PATH_TO_ROM>
```

To display the available options:

```
$ cargo run --release --help
```

## Controls

### Controller mapping

| Button | Key |
| ------ | --- |
| UP     | Z   |
| DOWN   | S   |
| LEFT   | Q   |
| RIGHT  | D   |
| A      | I   |
| B      | O   |
| START  | X   |
| SELECT | C   |

### Emulator features

| Feature              | Key        |
| -------------------- | ---------- |
| Debugging mode       | E          |
| Choose debug palette | Left/Right |
| Reset CPU            | R          |