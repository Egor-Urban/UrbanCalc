slint::include_modules!();
use slint::format;

mod utils {
    pub mod system_utils;
    pub mod logger;
}

use crate::utils::logger::LOGGER;


fn handle_keyboard_input(key: &str, main_window: &MainWindow) {
    let button_info = match key {
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
    
    LOGGER.info(&format!("Keyboard input: {} -> Type: {}, ID: {}", key, button_info.0, button_info.1));
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
    

    let main_window_weak = main_window.as_weak();
    main_window.on_button_pressed(move |button_type, button_id| {
        let type_str = match button_type {
            ButtonType::Number => "NUMBER",
            ButtonType::Operator => "OPERATOR", 
            ButtonType::Function => "FUNCTION",
            ButtonType::Parenthesis => "PARENTHESIS",
            ButtonType::Decimal => "DECIMAL",
            ButtonType::Clear => "CLEAR",
            ButtonType::Equals => "EQUALS",
        };
        
        LOGGER.info(&format!("Calculator button pressed: Type: {}, ID: {}", type_str, button_id.as_str()));
    });
    

    /*
    main_window.window().on_key_pressed(move |key_event| {
        let main_window = main_window_weak.upgrade().unwrap();
        let key_string = key_event.text.to_string();
        handle_keyboard_input(&key_string, &main_window);
        slint::EventResult::Accept
    });
    */
    
    
    main_window.run()
}