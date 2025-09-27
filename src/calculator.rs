use crate::utils::logger::LOGGER;

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
        
        // Check dot
        let mut can_add_decimal = true;
        let mut i = self.expression.len();
        
        while i > 0 {
            i -= 1;
            let ch = self.expression.chars().nth(i).unwrap();
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
               self.expression.chars().last().map_or(false, |c| "+-×÷(".contains(c)) {
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
            let last_char = self.expression.chars().last().unwrap();
            
            if "+-×÷".contains(last_char) {
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
                   self.expression.chars().last().map_or(false, |c| "+-×÷(".contains(c)) {
                    self.expression.push('(');
                    self.parentheses_count += 1;
                }
            },
            "close-paren" => {
                if self.parentheses_count > 0 &&
                   self.expression.chars().last().map_or(false, |c| c.is_ascii_digit() || c == '.' || c == ')') {
                    self.expression.push(')');
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
                    self.last_result = result;
                    self.result = format_number(result);
                    self.should_reset_expression = true;
                },
                Err(_) => {
                    self.result = "Error".to_string();
                    self.should_reset_expression = true;
                }
            }
        }
    }

    pub fn backspace(&mut self) {
        if !self.expression.is_empty() {
            let last_char = self.expression.chars().last().unwrap();
            if last_char == '(' {
                self.parentheses_count -= 1;
            } else if last_char == ')' {
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
                self.result = format_number(result);
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
        
        let last_char = expr.chars().last().unwrap();
        if "+-×÷(".contains(last_char) {
            return Err("Incomplete expression".to_string());
        }
        
        let normalized = expr
            .replace('×', "*")
            .replace('÷', "/");
        
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

struct ExpressionParser {
    input: Vec<char>,
    pos: usize,
}

impl ExpressionParser {
    fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
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
                '+' => {
                    self.pos += 1;
                    result += self.parse_term()?;
                },
                '-' => {
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
                '*' => {
                    self.pos += 1;
                    result *= self.parse_factor()?;
                },
                '/' => {
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
        self.skip_whitespace();
        
        if self.pos >= self.input.len() {
            return Err("Unexpected end of expression".to_string());
        }
        
        match self.current_char() {
            '(' => {
                self.pos += 1;
                let result = self.parse_expression()?;
                if self.pos >= self.input.len() || self.current_char() != ')' {
                    return Err("Missing closing parenthesis".to_string());
                }
                self.pos += 1;
                Ok(result)
            },
            '-' => {
                self.pos += 1;
                Ok(-self.parse_factor()?)
            },
            '+' => {
                self.pos += 1;
                self.parse_factor()
            },
            _ => self.parse_number(),
        }
    }
    
    fn parse_number(&mut self) -> Result<f64, String> {
        let start = self.pos;
        
        while self.pos < self.input.len() {
            let ch = self.current_char();
            if ch.is_ascii_digit() || ch == '.' {
                self.pos += 1;
            } else {
                break;
            }
        }
        
        if start == self.pos {
            return Err("Expected number".to_string());
        }
        
        let number_str: String = self.input[start..self.pos].iter().collect();
        number_str.parse::<f64>().map_err(|_| "Invalid number".to_string())
    }
    
    fn current_char(&self) -> char {
        if self.pos < self.input.len() {
            self.input[self.pos]
        } else {
            '\0'
        }
    }
    
    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() && self.current_char().is_whitespace() {
            self.pos += 1;
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