const COMMANDS: &[&str] = &[
    "start_listening",
    "stop_listening",
    "has_text",
    "has_rtf",
    "has_html",
    "has_image",
    "has_files",
    "read_text",
    "read_rtf",
    "read_html",
    "read_image",
    "read_files",
    "write_text",
    "write_rtf",
    "write_html",
    "write_image",
    "write_files",
    "clear",
    "get_default_save_image_path",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
