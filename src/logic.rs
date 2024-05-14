#[derive(Clone, Copy)]
pub enum Operation {
    Division,
    Times,
    Minus,
    Plus,
}

#[derive(Clone, Copy)]
pub enum ButtonType {
    Reset,
    Sign,
    Percent,
    Comma,
    Equal,
    Number(u8),
    Aritmethic(Operation),
}

pub struct Logic {
    first_value: f64,
    second_value: Option<f64>,
    operation: Option<Operation>,
    use_comma: bool,
}

impl Logic {
    pub fn new() -> Self {
        Self {
            first_value: 0.0,
            second_value: None,
            operation: None,
            use_comma: false,
        }
    }

    pub fn handle_key_input(&mut self, key_input: &str) {
        if let Ok(num) = key_input.parse::<u8>() {
            self.on_button_pressed(ButtonType::Number(num))
        } else {
            match key_input {
                "/" => self.on_button_pressed(ButtonType::Aritmethic(Operation::Division)),
                "*" => self.on_button_pressed(ButtonType::Aritmethic(Operation::Times)),
                "-" => self.on_button_pressed(ButtonType::Aritmethic(Operation::Minus)),
                "+" => self.on_button_pressed(ButtonType::Aritmethic(Operation::Plus)),
                "enter" => self.on_button_pressed(ButtonType::Equal),
                "=" => self.on_button_pressed(ButtonType::Equal),
                "," => self.on_button_pressed(ButtonType::Comma),
                "." => self.on_button_pressed(ButtonType::Comma),
                "%" => self.on_button_pressed(ButtonType::Percent),
                "backspace" => self.on_button_pressed(ButtonType::Reset),
                _ => {}
            }
        }
    }

    pub fn get_display_value(&self) -> f64 {
        self.second_value.unwrap_or(self.first_value)
    }

    pub fn on_button_pressed(&mut self, button_type: ButtonType) {
        match button_type {
            ButtonType::Number(num) => self.add_number(num),
            ButtonType::Equal => self.get_result(),
            ButtonType::Reset => self.set_result(0.0),
            ButtonType::Percent => self.change_current_value(&|n| n / 100.0),
            ButtonType::Comma => self.use_comma = true,
            ButtonType::Sign => self.change_current_value(&|n| n * -1.0),
            ButtonType::Aritmethic(operation) => {
                self.use_comma = false;
                self.operation = Some(operation)
            }
        }
    }

    fn append_digit(&self, value: f64, digit: u8) -> f64 {
        let digit_str = digit.to_string();
        let mut value_str = value.to_string();

        // Check if there's a decimal in the string, if not, add it
        if !value_str.contains('.') & self.use_comma {
            value_str.push('.');
        }

        // Append the digit to the string
        value_str.push_str(&digit_str);

        // Parse the string back to f64
        value_str.parse::<f64>().unwrap()
    }

    fn change_current_value(&mut self, f: &dyn Fn(f64) -> f64) {
        self.set_result(f(self.get_display_value()));
    }

    fn set_result(&mut self, n: f64) {
        self.first_value = n;
        self.second_value = None;
        self.operation = None;
        self.use_comma = n % 1.0 != 0.0;
    }

    fn get_result(&mut self) {
        if let (Some(op), Some(second_value)) = (self.operation, self.second_value) {
            let result = match op {
                Operation::Plus => self.first_value + second_value,
                Operation::Minus => self.first_value - second_value,
                Operation::Times => self.first_value * second_value,
                Operation::Division => self.first_value / second_value,
            };

            self.set_result(result);
        }
    }

    fn add_number(&mut self, n: u8) -> () {
        match self.operation {
            Some(_) => match self.second_value {
                Some(value) => self.second_value = Some(self.append_digit(value, n)),
                None => self.second_value = Some(n as f64),
            },
            None => self.first_value = self.append_digit(self.first_value, n),
        }
    }
}
