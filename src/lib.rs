use tauri::{
    generate_handler,
    plugin::{Builder, TauriPlugin},
    Runtime,
};

mod commands;

pub use commands::*;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("clipboard-x")
        .invoke_handler(generate_handler![
            commands::start_listening,
            commands::stop_listening,
            commands::has_text,
            commands::has_rtf,
            commands::has_html,
            commands::has_image,
            commands::has_files,
            commands::read_text,
            commands::read_rtf,
            commands::read_html,
            commands::read_image,
            commands::read_files,
            commands::write_text,
            commands::write_rtf,
            commands::write_html,
            commands::write_image,
            commands::write_files,
            commands::clear,
            commands::get_default_save_image_path,
        ])
        .build()
}
