slint::include_modules!();

mod utils {
    pub mod system_utils; // system_utils.rs
    pub mod logger;
}
use crate::utils::logger::LOGGER;

fn main() -> Result<(), slint::PlatformError> {
    LOGGER.info("App started");

    let os = utils::system_utils::get_os();
    LOGGER.info(&format!("Platform: {os}"));

    let main_window = MainWindow::new()?;
    main_window.run()
}