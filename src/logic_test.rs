use crate::logic::*;

#[test]
fn test_double_digit_input() {
    let mut logic = Logic::new();

    logic.on_button_pressed(ButtonType::Number(2));
    logic.on_button_pressed(ButtonType::Number(5));

    assert_eq!(logic.get_display_value(), 25.0)
}

#[test]
fn test_input_after_operator() {
    let mut logic = Logic::new();

    logic.on_button_pressed(ButtonType::Number(2));
    logic.on_button_pressed(ButtonType::Number(5));
    logic.on_button_pressed(ButtonType::Aritmethic(Operation::Division));

    assert_eq!(logic.get_display_value(), 25.0)
}

#[test]
fn test_input_after_operator_and_another_digit() {
    let mut logic = Logic::new();

    logic.on_button_pressed(ButtonType::Number(2));
    logic.on_button_pressed(ButtonType::Number(5));
    logic.on_button_pressed(ButtonType::Aritmethic(Operation::Division));
    logic.on_button_pressed(ButtonType::Number(9));

    assert_eq!(logic.get_display_value(), 9.0)
}

#[test]
fn test_input_sum_result() {
    let mut logic = Logic::new();

    logic.on_button_pressed(ButtonType::Number(2));
    logic.on_button_pressed(ButtonType::Number(5));
    logic.on_button_pressed(ButtonType::Aritmethic(Operation::Plus));
    logic.on_button_pressed(ButtonType::Number(9));
    logic.on_button_pressed(ButtonType::Equal);

    assert_eq!(logic.get_display_value(), 34.0)
}

#[test]
fn test_input_subtract_result() {
    let mut logic = Logic::new();

    logic.on_button_pressed(ButtonType::Number(2));
    logic.on_button_pressed(ButtonType::Number(5));
    logic.on_button_pressed(ButtonType::Aritmethic(Operation::Minus));
    logic.on_button_pressed(ButtonType::Number(5));
    logic.on_button_pressed(ButtonType::Equal);

    assert_eq!(logic.get_display_value(), 20.0)
}

#[test]
fn test_input_division_result() {
    let mut logic = Logic::new();

    logic.on_button_pressed(ButtonType::Number(2));
    logic.on_button_pressed(ButtonType::Number(5));
    logic.on_button_pressed(ButtonType::Aritmethic(Operation::Division));
    logic.on_button_pressed(ButtonType::Number(5));
    logic.on_button_pressed(ButtonType::Equal);

    assert_eq!(logic.get_display_value(), 5.0)
}

#[test]
fn test_input_division_with_floating_result() {
    let mut logic = Logic::new();

    logic.on_button_pressed(ButtonType::Number(2));
    logic.on_button_pressed(ButtonType::Number(5));
    logic.on_button_pressed(ButtonType::Aritmethic(Operation::Division));
    logic.on_button_pressed(ButtonType::Number(2));
    logic.on_button_pressed(ButtonType::Equal);

    assert_eq!(logic.get_display_value(), 12.5)
}

#[test]
fn test_input_multiply_result() {
    let mut logic = Logic::new();

    logic.on_button_pressed(ButtonType::Number(2));
    logic.on_button_pressed(ButtonType::Number(5));
    logic.on_button_pressed(ButtonType::Aritmethic(Operation::Times));
    logic.on_button_pressed(ButtonType::Number(2));
    logic.on_button_pressed(ButtonType::Equal);

    assert_eq!(logic.get_display_value(), 50.0)
}

#[test]
fn test_input_sum_of_floating_values_result() {
    let mut logic = Logic::new();

    logic.on_button_pressed(ButtonType::Number(2));
    logic.on_button_pressed(ButtonType::Number(5));
    logic.on_button_pressed(ButtonType::Comma);
    logic.on_button_pressed(ButtonType::Number(5));
    logic.on_button_pressed(ButtonType::Aritmethic(Operation::Plus));
    logic.on_button_pressed(ButtonType::Number(2));
    logic.on_button_pressed(ButtonType::Equal);

    assert_eq!(logic.get_display_value(), 27.5)
}

#[test]
fn test_reset() {
    let mut logic = Logic::new();

    logic.on_button_pressed(ButtonType::Number(2));
    logic.on_button_pressed(ButtonType::Number(5));
    logic.on_button_pressed(ButtonType::Reset);

    assert_eq!(logic.get_display_value(), 0.0)
}

#[test]
fn test_sign() {
    let mut logic = Logic::new();

    logic.on_button_pressed(ButtonType::Number(9));
    logic.on_button_pressed(ButtonType::Sign);
    logic.on_button_pressed(ButtonType::Aritmethic(Operation::Plus));
    logic.on_button_pressed(ButtonType::Number(5));
    logic.on_button_pressed(ButtonType::Equal);

    assert_eq!(logic.get_display_value(), -4.0)
}

#[test]
fn test_percentage() {
    let mut logic = Logic::new();

    logic.on_button_pressed(ButtonType::Number(9));
    logic.on_button_pressed(ButtonType::Percent);

    assert_eq!(logic.get_display_value(), 0.09)
}

#[test]
fn test_keyboard_sum() {
    let mut logic = Logic::new();

    logic.handle_key_input("9");
    logic.handle_key_input("2");
    logic.handle_key_input("+");
    logic.handle_key_input("2");
    logic.handle_key_input("enter");

    assert_eq!(logic.get_display_value(), 94.0)
}

#[test]
fn test_keyboard_subtract_with_equal() {
    let mut logic = Logic::new();

    logic.handle_key_input("9");
    logic.handle_key_input("2");
    logic.handle_key_input("-");
    logic.handle_key_input("2");
    logic.handle_key_input("=");

    assert_eq!(logic.get_display_value(), 90.0)
}

#[test]
fn test_keyboard_division() {
    let mut logic = Logic::new();

    logic.handle_key_input("1");
    logic.handle_key_input("2");
    logic.handle_key_input("/");
    logic.handle_key_input("2");
    logic.handle_key_input("enter");

    assert_eq!(logic.get_display_value(), 6.0)
}

#[test]
fn test_keyboard_multiplication() {
    let mut logic = Logic::new();

    logic.handle_key_input("1");
    logic.handle_key_input("2");
    logic.handle_key_input("*");
    logic.handle_key_input("2");
    logic.handle_key_input("enter");

    assert_eq!(logic.get_display_value(), 24.0)
}

#[test]
fn test_floating_with_dot() {
    let mut logic = Logic::new();

    logic.handle_key_input("1");
    logic.handle_key_input(".");
    logic.handle_key_input("2");
    logic.handle_key_input("+");
    logic.handle_key_input("2");
    logic.handle_key_input("enter");

    assert_eq!(logic.get_display_value(), 3.2)
}

#[test]
fn test_floating_with_comma() {
    let mut logic = Logic::new();

    logic.handle_key_input("1");
    logic.handle_key_input(",");
    logic.handle_key_input("2");
    logic.handle_key_input("+");
    logic.handle_key_input("2");
    logic.handle_key_input("enter");

    assert_eq!(logic.get_display_value(), 3.2)
}

#[test]
fn test_clear_with_backspace() {
    let mut logic = Logic::new();

    logic.handle_key_input("1");
    logic.handle_key_input("backspace");

    assert_eq!(logic.get_display_value(), 0.0)
}

#[test]
fn test_percentage_with_keyboard() {
    let mut logic = Logic::new();

    logic.handle_key_input("1");
    logic.handle_key_input("%");

    assert_eq!(logic.get_display_value(), 0.01)
}
