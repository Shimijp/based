# Based

A desktop number base converter built with Rust and [eframe/egui](https://github.com/emilk/egui). Convert between hexadecimal, decimal, and binary with a selected integer width and optional signed interpretation.

## Run

Install Rust and Cargo with support for the Rust 2024 edition (Rust 1.85 or newer; dependencies may require a newer toolchain). From the project directory, launch the app:

```sh
cargo run
```

To build an optimized executable:

```sh
cargo build --release
```

The executable is placed in `target/release/base_convert` (`base_convert.exe` on Windows). Running the app requires a graphical desktop environment.

## Usage

1. Enter a number and choose its base under **Select base**.
2. Choose **Byte**, **Two Byte**, **Four Byte**, or **Eight Byte** for an 8-, 16-, 32-, or 64-bit value.
3. Enable **Signed** to interpret the value as a two's complement signed integer.
4. Choose the **Result base** and click **Convert**.

Use **Select mode** to switch between light and dark themes.

### Input and output

- Hexadecimal input accepts an optional `0x` or `0X` prefix; binary accepts `0b` or `0B`.
- Leading and trailing whitespace is ignored.
- Signed decimal input must fit the selected signed range. Hexadecimal and binary inputs represent nonnegative bit patterns, including when **Signed** is enabled.
- Hexadecimal output uses an uppercase, zero-padded representation with a `0x` prefix. Binary output is zero-padded without a prefix. Both use the selected bit width.
- Invalid input and values outside the selected range display an error in the result area.

Examples with **Byte** selected:

| Input | Input base | Signed | Result base | Output |
| --- | --- | --- | --- | --- |
| `FF` | Hexadecimal | No | Decimal | `255` |
| `0xFF` | Hexadecimal | Yes | Decimal | `-1` |
| `-1` | Decimal | Yes | Hexadecimal | `0xFF` |
| `10` | Decimal | No | Binary | `00001010` |

## Project layout

- `src/main.rs`: native application entry point and window setup.
- `src/base.rs`: interface, themes, and conversion logic.
- `Cargo.toml`: package metadata and dependencies.
