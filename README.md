# CHIP-8 Emulator

A CHIP-8 emulator written in Rust. It uses a 64×32 display and maps the CHIP-8
hex keypad to a QWERTY keyboard.

## Requirements

- Rust and Cargo (installable with [rustup](https://rustup.rs/))
- A graphical desktop environment supported by
  [minifb](https://github.com/emoon/rust_minifb)

## Run

Start the included IBM Logo ROM:

```sh
cargo run -- "data/IBM Logo"
```

You can also pass a path to another CHIP-8 ROM:

```sh
cargo run -- path/to/rom
```

If no path is provided, the emulator uses `data/IBM Logo`.

## Keypad

| CHIP-8  | Keyboard |
| :-----: | :------: |
| 1 2 3 C | 1 2 3 4  |
| 4 5 6 D | Q W E R  |
| 7 8 9 E | A S D F  |
| A 0 B F | Z X C V  |

Close the emulator window to exit.

## Implementation

The emulator includes 4 KiB of memory, the CHIP-8 font set, sixteen 8-bit
registers, the index register, stack and program counter, keypad input, sprite
drawing with collision detection, and delay and sound timer countdowns. Its
instruction implementation covers the common original CHIP-8 operations,
including control flow, arithmetic and logic, drawing, keypad tests, timers,
and memory transfers.

Timers tick at 60 Hz. The sound timer is implemented as state only; the emulator
does not currently produce audio.

## Development

Run the test suite and formatting check with:

```sh
cargo test
cargo fmt --check
```
