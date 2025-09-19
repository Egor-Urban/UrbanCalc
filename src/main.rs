slint::include_modules!();
use slint::format;

mod utils {
    pub mod system_utils; // system_utils.rs
    pub mod logger;
}

use crate::utils::logger::LOGGER;




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

    main_window.run()
}