slint::include_modules!();


mod utils {
    pub mod system_utils;
    pub mod logger;
}


mod calculator;


use crate::utils::logger::LOGGER;
use crate::calculator::{Calculator, handle_calculator_input};



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
    

    // calc init
    let mut calculator = Calculator::new();
    main_window.set_expression_text(slint::SharedString::from(""));
    main_window.set_result_text(slint::SharedString::from("0"));
    
    let calculator_rc = std::rc::Rc::new(std::cell::RefCell::new(calculator));
    let calculator_for_buttons = calculator_rc.clone();
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
        
        handle_calculator_input(button_id.as_str(), &mut calc);
        
        // Update UI after calculation
        main_window.set_expression_text(slint::SharedString::from(calc.get_expression()));
        main_window.set_result_text(slint::SharedString::from(calc.get_result()));
        
        LOGGER.info(&format!("Calculator button pressed: Type: {}, ID: {}", type_str, button_id.as_str()));
    });
    
    main_window.run()
}