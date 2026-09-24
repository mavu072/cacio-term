<p align="center">
  <br />
  <a href="https://codeberg.org/mavu072/cacio-term">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/get-it-on-codeberg-dark.png">
      <img src="assets/get-it-on-codeberg.png" width="300px">
    </picture>
  </a>
</p>

<p align="center">
  <strong>cacio-term</strong> TUI watch emulator built using Rust with ratatui, and crossterm
</p>

<p align="center">
  <img alt="GitHub License" src="https://img.shields.io/github/license/mavu072/cacio-term">
  <img alt="GitHub top language" src="https://img.shields.io/github/languages/top/mavu072/cacio-term">
  <img alt="CI" src="https://github.com/mavu072/cacio-term/actions/workflows/ci.yaml/badge.svg" />
  <img alt="Casio Illuminator" src="https://img.shields.io/badge/Casio-Illuminator-green">
</p>
<br />

# cacio-term

A high-performance Terminal User Interface (TUI) that emulates the iconic **Casio Illuminator** digital watch, built in **Rust** using the **`ratatui`** framework and the **`crossterm`** backend.

## Features

* **Cross-Platform Compatibility:** Runs on Linux, macOS, and Windows.
* **Real-Time Module:** Accurate clock mirroring the Casio digital layout (Time, Date, Day of the Week).
* **Mode Cycling:** Supports Timekeeping, Alarm, Dual Time and Stopwatch watch modes.
* **Illuminator Backlight:** Inverts and recolors TUI widgets to simulate the retro LCD glow.

> [!NOTE]
> This software is still early development, and not all features are available yet.

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
git clone https://codeberg.org/mavu072/cacio-term.git
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

## AI Usage

This entire codebase was architected, written, and debugged entirely by hand. No AI code generation, no LLM autocomplete. The choice to go 100% No-AI is entirely educational, forcing me to truly learn the fundamentals of Rust.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
