
use crate::utils::logger::LOGGER;
use std::collections::HashMap;

pub struct KeyboardHandler {
    key_mappings: HashMap<String, (&'static str, &'static str)>,
}

impl KeyboardHandler {
    pub fn new() -> Self {
        let mut key_mappings = HashMap::new();
        
        for i in 0..=9 {
            key_mappings.insert(i.to_string(), ("NUMBER", match i {
                0 => "0", 1 => "1", 2 => "2", 3 => "3", 4 => "4",
                5 => "5", 6 => "6", 7 => "7", 8 => "8", 9 => "9",
                _ => unreachable!()
            }));
        }
        
        key_mappings.insert("+".to_string(), ("OPERATOR", "plus"));
        key_mappings.insert("-".to_string(), ("OPERATOR", "minus"));
        key_mappings.insert("*".to_string(), ("OPERATOR", "multiply"));
        key_mappings.insert("/".to_string(), ("OPERATOR", "divide"));
        key_mappings.insert("=".to_string(), ("EQUALS", "equals"));
        key_mappings.insert("Enter".to_string(), ("EQUALS", "equals"));
        key_mappings.insert(".".to_string(), ("DECIMAL", "decimal"));
        key_mappings.insert(",".to_string(), ("DECIMAL", "decimal"));
        key_mappings.insert("(".to_string(), ("PARENTHESIS", "open-paren"));
        key_mappings.insert(")".to_string(), ("PARENTHESIS", "close-paren"));
        key_mappings.insert("Backspace".to_string(), ("FUNCTION", "backspace"));
        key_mappings.insert("Delete".to_string(), ("CLEAR", "clear"));
        key_mappings.insert("c".to_string(), ("CLEAR", "clear"));
        key_mappings.insert("C".to_string(), ("CLEAR", "clear"));
        key_mappings.insert("Escape".to_string(), ("FUNCTION", "settings"));
        
        Self { key_mappings }
    }
    
    pub fn handle_key(&self, key: &str) -> Option<(&'static str, &'static str)> {
        if let Some(&(button_type, button_id)) = self.key_mappings.get(key) {
            LOGGER.info(&format!("Keyboard input: '{}' -> Type: {}, ID: {}", key, button_type, button_id));
            Some((button_type, button_id))
        } else {
            LOGGER.info(&format!("Unknown keyboard input: '{}'", key));
            None
        }
    }
    
    pub fn get_supported_keys(&self) -> Vec<String> {
        self.key_mappings.keys().cloned().collect()
    }
    
    pub fn add_key_mapping(&mut self, key: String, button_type: &'static str, button_id: &'static str) {
        self.key_mappings.insert(key, (button_type, button_id));
    }
}

pub fn simulate_calculator_button_press(button_type: &str, button_id: &str) {
    LOGGER.info(&format!("Simulating calculator button press: Type: {}, ID: {}", button_type, button_id));
    
}