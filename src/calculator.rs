use crate::utils::logger::LOGGER;

const OPERATORS: &str = "+-×÷";
const OPEN_PAREN: char = '(';
const CLOSE_PAREN: char = ')';

pub struct Calculator {
    expression: String,
    result: String,
    last_result: f64,
    should_reset_expression: bool,
    parentheses_count: i32,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            expression: String::new(),
            result: String::from("0"),
            last_result: 0.0,
            should_reset_expression: false,
            parentheses_count: 0,
        }
    }

    pub fn add_digit(&mut self, digit: &str) {
        if self.should_reset_expression {
            self.expression.clear();
            self.should_reset_expression = false;
        }
        
        self.expression.push_str(digit);
        self.update_result();
    }

    pub fn add_decimal(&mut self) {
        if self.should_reset_expression {
            self.expression.clear();
            self.should_reset_expression = false;
        }
        
        let mut can_add_decimal = true;
        let mut i = self.expression.len();
        
        while i > 0 {
            i -= 1;
            let ch = self.expression.as_bytes()[i] as char;
            if ch == '.' {
                can_add_decimal = false;
                break;
            }
            if !ch.is_ascii_digit() {
                break;
            }
        }
        
        if can_add_decimal {
            if self.expression.is_empty() || 
               OPERATORS.contains(self.expression.as_bytes().last().map_or('\0', |&b| b as char)) ||
               self.expression.as_bytes().last().map_or(false, |&b| b as char == OPEN_PAREN) {
                self.expression.push_str("0.");
            } else {
                self.expression.push('.');
            }
            self.update_result();
        }
    }

    pub fn add_operator(&mut self, operator: &str) {
        if self.should_reset_expression {
            self.expression = self.result.clone();
            self.should_reset_expression = false;
        }
        
        if !self.expression.is_empty() {
            let last_char = char::from(*self.expression.as_bytes().last().unwrap());
            
            if OPERATORS.contains(last_char) {
                self.expression.pop();
            }
            
            let op_symbol = match operator {
                "plus" => "+",
                "minus" => "-",
                "multiply" => "×",
                "divide" => "÷",
                _ => return,
            };
            
            self.expression.push_str(op_symbol);
        } else if operator == "minus" {
            self.expression.push('-');
        }
    }

    pub fn add_parenthesis(&mut self, paren_type: &str) {
        if self.should_reset_expression {
            self.expression.clear();
            self.should_reset_expression = false;
        }
        
        match paren_type {
            "open-paren" => {
                if self.expression.is_empty() || 
                   OPERATORS.contains(self.expression.as_bytes().last().map_or('\0', |&b| b as char)) ||
                   self.expression.as_bytes().last().map_or(false, |&b| b as char == OPEN_PAREN) {
                    self.expression.push(OPEN_PAREN);
                    self.parentheses_count += 1;
                }
            },
            "close-paren" => {
                if self.parentheses_count > 0 &&
                   self.expression.as_bytes().last().map_or(false, |&b| {
                       let c = b as char;
                       c.is_ascii_digit() || c == '.' || c == CLOSE_PAREN
                   }) {
                    self.expression.push(CLOSE_PAREN);
                    self.parentheses_count -= 1;
                }
            },
            _ => {}
        }
        
        self.update_result();
    }

    pub fn calculate(&mut self) {
        if !self.expression.is_empty() {
            match self.evaluate_expression(&self.expression) {
                Ok(result) => {
                    if result.is_nan() || result.is_infinite() {
                        self.result = "Error".to_string();
                    } else {
                        self.last_result = result;
                        self.result = format_number(result);
                    }
                    self.should_reset_expression = true;
                },
                Err(e) => {
                    self.result = format!("Error: {}", e);
                    self.should_reset_expression = true;
                }
            }
        }
    }

    pub fn backspace(&mut self) {
        if !self.expression.is_empty() {
            let last_char = char::from(*self.expression.as_bytes().last().unwrap());
            if last_char == OPEN_PAREN {
                self.parentheses_count -= 1;
            } else if last_char == CLOSE_PAREN {
                self.parentheses_count += 1;
            }
            self.expression.pop();
            self.update_result();
        }
    }

    pub fn clear(&mut self) {
        self.expression.clear();
        self.result = "0".to_string();
        self.last_result = 0.0;
        self.should_reset_expression = false;
        self.parentheses_count = 0;
    }

    pub fn get_expression(&self) -> String {
        self.expression.clone()
    }

    pub fn get_result(&self) -> String {
        self.result.clone()
    }

    fn update_result(&mut self) {
        if self.expression.is_empty() {
            self.result = "0".to_string();
            return;
        }

        match self.evaluate_expression(&self.expression) {
            Ok(result) => {
                if result.is_nan() || result.is_infinite() {
                    self.result = "0".to_string();
                } else {
                    self.result = format_number(result);
                }
            },
            Err(_) => {
                self.result = "0".to_string();
            }
        }
    }

    fn evaluate_expression(&self, expr: &str) -> Result<f64, String> {
        if expr.is_empty() {
            return Ok(0.0);
        }
        
        let last_char = char::from(*expr.as_bytes().last().unwrap());
        if OPERATORS.contains(last_char) || last_char == OPEN_PAREN || last_char == '.' {
            return Err("Incomplete expression".to_string());
        }
        
        let normalized = expr.replace('×', "*").replace('÷', "/");
        self.parse_expression(&normalized)
    }
    
    fn parse_expression(&self, expr: &str) -> Result<f64, String> {
        let expr = expr.replace(' ', "");
        self.evaluate_string(&expr)
    }
    
    fn evaluate_string(&self, expr: &str) -> Result<f64, String> {
        if expr.is_empty() {
            return Ok(0.0);
        }
        
        let mut parser = ExpressionParser::new(expr);
        parser.parse()
    }
}

struct ExpressionParser<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> ExpressionParser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input: input.as_bytes(),
            pos: 0,
        }
    }
    
    fn parse(&mut self) -> Result<f64, String> {
        let result = self.parse_expression()?;
        if self.pos < self.input.len() {
            return Err("Unexpected character".to_string());
        }
        Ok(result)
    }
    
    fn parse_expression(&mut self) -> Result<f64, String> {
        let mut result = self.parse_term()?;
        
        while self.pos < self.input.len() {
            match self.current_char() {
                b'+' => {
                    self.pos += 1;
                    result += self.parse_term()?;
                },
                b'-' => {
                    self.pos += 1;
                    result -= self.parse_term()?;
                },
                _ => break,
            }
        }
        
        Ok(result)
    }
    
    fn parse_term(&mut self) -> Result<f64, String> {
        let mut result = self.parse_factor()?;
        
        while self.pos < self.input.len() {
            match self.current_char() {
                b'*' => {
                    self.pos += 1;
                    result *= self.parse_factor()?;
                },
                b'/' => {
                    self.pos += 1;
                    let divisor = self.parse_factor()?;
                    if divisor == 0.0 {
                        return Err("Division by zero".to_string());
                    }
                    result /= divisor;
                },
                _ => break,
            }
        }
        
        Ok(result)
    }
    
    fn parse_factor(&mut self) -> Result<f64, String> {
        if self.pos >= self.input.len() {
            return Err("Unexpected end of expression".to_string());
        }
        
        match self.current_char() {
            b'(' => {
                self.pos += 1;
                let result = self.parse_expression()?;
                if self.pos >= self.input.len() || self.current_char() != b')' {
                    return Err("Missing closing parenthesis".to_string());
                }
                self.pos += 1;
                Ok(result)
            },
            b'-' => {
                self.pos += 1;
                Ok(-self.parse_factor()?)
            },
            b'+' => {
                self.pos += 1;
                self.parse_factor()
            },
            _ => self.parse_number(),
        }
    }
    
    fn parse_number(&mut self) -> Result<f64, String> {
        let start = self.pos;
        let is_negative = if self.pos < self.input.len() && self.current_char() == b'-' {
            self.pos += 1;
            true
        } else {
            false
        };

        let has_digits = self.pos < self.input.len() && self.current_char().is_ascii_digit();
        while self.pos < self.input.len() && self.current_char().is_ascii_digit() {
            self.pos += 1;
        }

        let has_decimal = if self.pos < self.input.len() && self.current_char() == b'.' {
            self.pos += 1;
            while self.pos < self.input.len() && self.current_char().is_ascii_digit() {
                self.pos += 1;
            }
            true
        } else {
            false
        };

        let has_exponent = if self.pos < self.input.len() && (self.current_char() == b'e' || self.current_char() == b'E') {
            self.pos += 1;
            if self.pos < self.input.len() && (self.current_char() == b'+' || self.current_char() == b'-') {
                self.pos += 1;
            }
            let exp_start = self.pos;
            while self.pos < self.input.len() && self.current_char().is_ascii_digit() {
                self.pos += 1;
            }
            exp_start != self.pos
        } else {
            false
        };

        if start == self.pos || (!has_digits && !has_exponent && has_decimal) {
            return Err("Invalid number format".to_string());
        }

        let number_str = std::str::from_utf8(&self.input[start..self.pos]).map_err(|_| "Invalid UTF-8".to_string())?;
        let number = number_str.parse::<f64>().map_err(|_| "Invalid number".to_string())?;
        Ok(if is_negative { -number } else { number })
    }
    
    fn current_char(&self) -> u8 {
        if self.pos < self.input.len() {
            self.input[self.pos]
        } else {
            0
        }
    }
}

fn format_number(num: f64) -> String {
    if num.is_nan() {
        return "Error".to_string();
    }
    if num.is_infinite() {
        return if num.is_sign_positive() { "Infinity" } else { "-Infinity" }.to_string();
    }

    if num.fract().abs() < 1e-10 && num.abs() < 1e15 {
        return format!("{}", num as i64);
    }

    let formatted = format!("{:.8}", num);
    let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
    if trimmed.len() > 12 || num.abs() > 1e15 || num.abs() < 1e-4 {
        format!("{:.6e}", num).trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        trimmed.to_string()
    }
}

pub fn handle_calculator_input(button_id: &str, calculator: &mut Calculator) {
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
        "open-paren" | "close-paren" => {
            calculator.add_parenthesis(button_id);
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
}