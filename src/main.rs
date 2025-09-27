slint::include_modules!();

mod utils {
    pub mod system_utils;
    pub mod logger;
}

use crate::utils::logger::LOGGER;

struct Calculator {
    display: String,
    current_input: String,
    last_operator: String,
    result: f64,
    should_reset: bool,
}

impl Calculator {
    fn new() -> Self {
        Self {
            display: String::from("0"),
            current_input: String::new(),
            last_operator: String::new(),
            result: 0.0,
            should_reset: false,
        }
    }

    fn add_digit(&mut self, digit: &str) {
        if self.should_reset {
            self.current_input.clear();
            self.should_reset = false;
        }
        
        if self.current_input == "0" && digit != "." {
            self.current_input = digit.to_string();
        } else {
            self.current_input.push_str(digit);
        }
        
        self.display = self.current_input.clone();
    }

    fn add_decimal(&mut self) {
        if self.should_reset {
            self.current_input = "0".to_string();
            self.should_reset = false;
        }
        
        if !self.current_input.contains('.') {
            if self.current_input.is_empty() {
                self.current_input = "0.".to_string();
            } else {
                self.current_input.push('.');
            }
        }
        
        self.display = self.current_input.clone();
    }

    fn add_operator(&mut self, operator: &str) {
        if !self.current_input.is_empty() {
            if let Ok(value) = self.current_input.parse::<f64>() {
                if !self.last_operator.is_empty() {
                    self.calculate();
                } else {
                    self.result = value;
                }
            }
        }
        
        self.last_operator = operator.to_string();
        self.should_reset = true;
    }

    fn calculate(&mut self) {
        if !self.current_input.is_empty() && !self.last_operator.is_empty() {
            if let Ok(value) = self.current_input.parse::<f64>() {
                match self.last_operator.as_str() {
                    "plus" => self.result += value,
                    "minus" => self.result -= value,
                    "multiply" => self.result *= value,
                    "divide" => {
                        if value != 0.0 {
                            self.result /= value;
                        } else {
                            self.display = "Error".to_string();
                            self.clear();
                            return;
                        }
                    },
                    _ => {}
                }
                
                self.display = format_number(self.result);
                self.current_input = self.display.clone();
                self.last_operator.clear();
                self.should_reset = true;
            }
        }
    }

    fn backspace(&mut self) {
        if !self.current_input.is_empty() {
            self.current_input.pop();
            if self.current_input.is_empty() {
                self.current_input = "0".to_string();
            }
            self.display = self.current_input.clone();
        }
    }

    fn clear(&mut self) {
        self.display = "0".to_string();
        self.current_input.clear();
        self.last_operator.clear();
        self.result = 0.0;
        self.should_reset = false;
    }

    fn get_display(&self) -> String {
        if self.display.is_empty() {
            "0".to_string()
        } else {
            self.display.clone()
        }
    }
}

fn format_number(num: f64) -> String {
    if num.fract() == 0.0 && num.abs() < 1e15 {
        format!("{}", num as i64)
    } else {
        let formatted = format!("{}", num);
        if formatted.len() > 12 {
            format!("{:.6e}", num)
        } else {
            formatted
        }
    }
}

fn handle_keyboard_input(key: &str, main_window: &MainWindow, calculator: &mut Calculator) {
    let (button_type, button_id) = match key {
        "0" => ("NUMBER", "0"),
        "1" => ("NUMBER", "1"),
        "2" => ("NUMBER", "2"),
        "3" => ("NUMBER", "3"),
        "4" => ("NUMBER", "4"),
        "5" => ("NUMBER", "5"),
        "6" => ("NUMBER", "6"),
        "7" => ("NUMBER", "7"),
        "8" => ("NUMBER", "8"),
        "9" => ("NUMBER", "9"),
        
        "+" => ("OPERATOR", "plus"),
        "-" => ("OPERATOR", "minus"),
        "*" => ("OPERATOR", "multiply"),
        "/" => ("OPERATOR", "divide"),
        
        "=" | "Enter" => ("EQUALS", "equals"),
        "." | "," => ("DECIMAL", "decimal"),
        "(" => ("PARENTHESIS", "open-paren"),
        ")" => ("PARENTHESIS", "close-paren"),
        "Backspace" => ("FUNCTION", "backspace"),
        "Delete" | "c" | "C" => ("CLEAR", "clear"),
        "Escape" => ("FUNCTION", "settings"),
        
        _ => return, 
    };
    
    handle_calculator_input(button_id, calculator, main_window);
    
    LOGGER.info(&format!("Keyboard input: {} -> Type: {}, ID: {}", key, button_type, button_id));
}

fn handle_calculator_input(button_id: &str, calculator: &mut Calculator, main_window: &MainWindow) {
    match button_id {
        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
            calculator.add_digit(button_id);
        },
        "decimal" => {
            calculator.add_decimal();
        },
        "plus" | "minus" | "multiply" | "divide" => {
            calculator.add_operator(button_id);
        },
        "equals" => {
            calculator.calculate();
        },
        "backspace" => {
            calculator.backspace();
        },
        "clear" => {
            calculator.clear();
        },
        "settings" => {
            LOGGER.info("Settings button pressed");
        },
        _ => {}
    }
    
    main_window.set_display_text(slint::SharedString::from(calculator.get_display()));
}

fn main() -> Result<(), slint::PlatformError> {
    LOGGER.info("App started");
    
    let os = utils::system_utils::get_os();
    let theme = utils::system_utils::get_theme(os);
    
    LOGGER.info(&format!("Platform: {os}"));
    LOGGER.info(&format!("Theme: {theme}"));
    
    let main_window = MainWindow::new()?;
    let app_theme = AppTheme::get(&main_window);
    
    match theme {
        "dark" => {
            app_theme.set_background(slint::Color::from_rgb_u8(38, 38, 38));
            app_theme.set_text(slint::Color::from_rgb_u8(235, 235, 235));
        }
        "light" => {
            app_theme.set_background(slint::Color::from_rgb_u8(255, 255, 255));
            app_theme.set_text(slint::Color::from_rgb_u8(0, 0, 0));
        }
        _ => {}
    }
    
    // INIT
    let mut calculator = Calculator::new();
    main_window.set_display_text(slint::SharedString::from("0"));
    
    let calculator_rc = std::rc::Rc::new(std::cell::RefCell::new(calculator));
    let calculator_for_buttons = calculator_rc.clone();
    let calculator_for_keyboard = calculator_rc.clone();
    
    // keypress handle
    let main_window_weak_buttons = main_window.as_weak();
    main_window.on_button_pressed(move |button_type, button_id| {
        let main_window = main_window_weak_buttons.upgrade().unwrap();
        let mut calc = calculator_for_buttons.borrow_mut();
        
        let type_str = match button_type {
            ButtonType::Number => "NUMBER",
            ButtonType::Operator => "OPERATOR", 
            ButtonType::Function => "FUNCTION",
            ButtonType::Parenthesis => "PARENTHESIS",
            ButtonType::Decimal => "DECIMAL",
            ButtonType::Clear => "CLEAR",
            ButtonType::Equals => "EQUALS",
        };
        
        handle_calculator_input(button_id.as_str(), &mut calc, &main_window);
        
        LOGGER.info(&format!("Calculator button pressed: Type: {}, ID: {}", type_str, button_id.as_str()));
    });
    
    //TODO Physical jeyboard handling
    
    main_window.run()
}