# cacio-term

A high-performance Terminal User Interface (TUI) that recreates the iconic **Casio Illuminator** digital watch, handcrafted in **Rust** using the **`ratatui`** framework and the **`crossterm`** backend.

## 100% No-AI Project

This entire codebase was architected, written, and debugged entirely by hand. No AI code generation, no LLM autocomplete. The choice to go 100% No-AI is entirely educational, forcing me to truly learn the fundamentals of Rust.

## The Architecture

By pairing `ratatui` with `crossterm`, this project achieves cross-platform terminal control while the core emulation logic remains entirely hand-rolled:
* **Event Handling:** Leveraging `crossterm` to capture keyboard events for responsive button presses.
* **Custom Watch States:** State transitions to perfectly mimic the Casio modes.
* **TUI Layouts:** Layouts using `ratatui` primitives to mirror retro digital LCD segments and text placement.
* **LCD Light Simulation:** Custom style inverters that dynamically shift terminal colors to replicate the glow of a classic Illuminator LCD.

## Features

* **Real-Time Module:** High-accuracy clock mirroring the classic Casio digital layout (Time, Date, Day of the Week).
* **Cross-Platform Compatibility:** Runs flawlessly on Linux, macOS, and Windows thanks to the `crossterm` backend.
* **Mode Cycling:** Accurate watch state logic for switching between Timekeeping, Alarm, Dual Time and Stopwatch modes.
* **Illuminator Backlight:** Inverts and recolors TUI widgets to simulate the retro LCD glow.
* **Clean Rust Engine:** Fast performance, and strict type safety from `Rust`.

## Controls & Button Mapping

The application maps keyboard shortcuts to the physical buttons of a Casio watch:

| Keyboard Key | Watch Button | Function |
| :--- | :--- | :--- |
| `M` / `Tab` | **MODE** | Cycle through Time ➔ Alarm ➔ Stopwatch |
| `L` / `Space`| **LIGHT** | Trigger the Illuminator LCD light |
| `A` | **ADJUST** | Hold to enter setting mode (Hour/Minute adjustments) |
| `S` | **START/STOP** | Start/stop the stopwatch |
| `H` | **12/24H** | Toggle 12/24-hour formats |
| `Q` / `Esc` | — | Gracefully exit `cacio-term` |

## Installation

Ensure you have [Rust and Cargo installed](https://rust-lang.org), then clone and build directly from source:

```bash
git clone https://codeberg.org
cd cacio-term
cargo run --release
```

## Compiling 

Compile with `cargo build` and use `cargo run` to compile and run with one command:

```bash
cargo build
cargo run
```

## Running tests

Run tests with:

```bash
cargo test
```

## Contributing

All contributions are welcome! 

1. Fork the project repository.
2. Hand-code your feature or bug fix.
3. Open a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
