use crate::logic::*;

pub const BUTTONS: [ButtonType; 19] = [
    ButtonType::Reset,
    ButtonType::Sign,
    ButtonType::Percent,
    ButtonType::Aritmethic(Operation::Division),
    ButtonType::Number(7),
    ButtonType::Number(8),
    ButtonType::Number(9),
    ButtonType::Aritmethic(Operation::Times),
    ButtonType::Number(4),
    ButtonType::Number(5),
    ButtonType::Number(6),
    ButtonType::Aritmethic(Operation::Minus),
    ButtonType::Number(1),
    ButtonType::Number(2),
    ButtonType::Number(3),
    ButtonType::Aritmethic(Operation::Plus),
    ButtonType::Number(0),
    ButtonType::Comma,
    ButtonType::Equal,
];
