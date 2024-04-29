## Gpui Based Calculator

This is a simple calculator that can perform basic arithmetic operations. It is built using Rust and the Zed Gpui library. The calculator has a simple GUI that allows users to input numbers and operators using buttons. The calculator can perform addition, subtraction, multiplication, and division. It also has a clear button that allows users to clear the input field.

## Aim

The aim of the project is to demonstrate how to build a simple calculator using Rust and the Zed Gpui library. The calculator is a basic example of how to create a GUI application using Rust and Zed Gpui. It is intended to be a starting point for developers who want to build more complex GUI applications using Rust and Zed Gpui.

## Structure

The project is structured as follows:

- `src/main.rs`: The main entry point of the application. It contains the code for creating the GUI and handling user input.
- `src/root`: The root of the component containing the calculator buttons and input field.
- `src/button`: The button component that represents a button on the calculator.
- `src/consts`: This module contains constants used in the application.
- `src/logic`: This module contains the struct that stores and manipulates the calculator state.

## How to Run

Be sure to have rust installed on your machine. You can install rust by following the instructions on the [official rust website](https://www.rust-lang.org/tools/install).

```bash
cargo fetch
cargo run
```
