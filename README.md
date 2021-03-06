# Nesmulator GUI

This project is an example of how to handle a GUI with the [nesmulator-core](https://github.com/AntoineRR/nesmulator-core) crate.

## Current features

* [X] Display the game screen
* [X] A debugging view (display of pattern tables and palette) can be toggled
* [X] First Controller mapping for keyboard
* [X] CLI with various flags (see below)

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

To compile the project on Windows, you should follow the instructions [here](https://rustrepo.com/repo/AngryLawyer-rust-sdl2#windows-msvc) to make SDL2 (used for the sound) work.

To display the available options:

```
$ cargo run --release -- --help

USAGE:
    nesmulator-gui [OPTIONS] <FILE>

ARGS:
    <FILE>    Sets the nes file to run in the emulator

OPTIONS:
    -d, --debug <INT>       Turn debugging information on
    -h, --help              Print help information
    -l, --log               Display the CPU logs to the console
    -m, --state <FILE>      Specify a .data state file to load in the emulator
    -p, --palette <FILE>    Sets a palette from a .pal file
    -s, --save <FILE>       Specify a .sav file to load in the emulator. This works for games that originally provided a save system.
    -V, --version           Print version information
```

The debug level must be between 1 and 4.
The palette configuration file can be generated [here](https://bisqwit.iki.fi/utils/nespalette.php).

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

| Feature                 | Key        |
| ----------------------- | ---------- |
| Debugging mode          | E          |
| Choose debug palette    | Left/Right |
| Control emulation speed | Up/down    |
| Save current state      | M          |
| Save game               | L          |
| Reset CPU               | R          |

There is a difference between saving the state of the emulator and the game. Saving the state will let you restart the game exactly where you stopped it, while saving the game will work as in the original NES (you first have to save in game, then press the save button on the emulator).

## To do

* Add a configuration file
* Improve sound quality