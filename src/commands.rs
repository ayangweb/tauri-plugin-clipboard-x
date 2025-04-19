use clipboard_rs::{
    common::RustImage, Clipboard, ClipboardContent, ClipboardContext, ClipboardHandler,
    ClipboardWatcher, ClipboardWatcherContext, ContentFormat, RustImageData, WatcherShutdown,
};
use fs_extra::dir::get_size;
use std::{
    fs::create_dir_all,
    hash::{DefaultHasher, Hash, Hasher},
    path::PathBuf,
    sync::{Arc, Mutex, OnceLock},
    thread::spawn,
};
use tauri::{command, AppHandle, Emitter, Manager, Runtime};

static CLIPBOARD_MANAGER: OnceLock<ClipboardManager> = OnceLock::new();

pub struct ClipboardManager {
    ctx: Arc<Mutex<ClipboardContext>>,
    watcher: Arc<Mutex<Option<WatcherShutdown>>>,
}

struct ClipboardListen<R>
where
    R: Runtime,
{
    app_handle: AppHandle<R>,
}

impl ClipboardManager {
    pub fn new() -> Self {
        ClipboardManager {
            ctx: Arc::new(Mutex::new(ClipboardContext::new().unwrap())),
            watcher: Arc::default(),
        }
    }

    fn has(&self, format: ContentFormat) -> Result<bool, String> {
        self.ctx
            .lock()
            .map_err(|err| err.to_string())
            .map(|ctx| ctx.has(format))
    }

    fn write(&self, contents: Vec<ClipboardContent>) -> Result<(), String> {
        self.ctx
            .lock()
            .map_err(|err| err.to_string())?
            .set(contents)
            .map_err(|err| err.to_string())
    }
}

impl<R> ClipboardListen<R>
where
    R: Runtime,
{
    fn new(app_handle: AppHandle<R>) -> Self {
        Self { app_handle }
    }
}

impl<R> ClipboardHandler for ClipboardListen<R>
where
    R: Runtime,
{
    fn on_clipboard_change(&mut self) {
        let _ = self
            .app_handle
            .emit("plugin:clipboard-x://clipboard_changed", ())
            .map_err(|err| err.to_string());
    }
}

#[derive(Debug, serde::Serialize)]
pub struct ReadImage {
    /// The path of the image.
    pub path: PathBuf,
    /// The size of the image in bytes.
    pub size: u64,
    /// The width of the image in pixels.
    pub width: u32,
    /// The height of the image in pixels.
    pub height: u32,
}

#[derive(Debug, serde::Serialize)]
pub struct ReadFile {
    /// The paths of the files.
    pub paths: Vec<String>,
    /// The size of the files in bytes.
    pub size: u64,
}

/// Get the global clipboard manager instance.
fn get_clipboard_manager() -> &'static ClipboardManager {
    CLIPBOARD_MANAGER.get_or_init(|| ClipboardManager::new())
}

/// Start listening for clipboard changes.
///
/// # Example
/// ```
/// use tauri_plugin_clipboard_x::start_listening;
///
/// start_listening(app_handle).await?;
///```
#[command]
pub async fn start_listening<R: Runtime>(app_handle: AppHandle<R>) -> Result<(), String> {
    let listener = ClipboardListen::new(app_handle.clone());

    let mut watcher = ClipboardWatcherContext::new().map_err(|err| err.to_string())?;

    let watcher_shutdown = watcher.add_handler(listener).get_shutdown_channel();

    let mut watcher_shutdown_state = get_clipboard_manager()
        .watcher
        .lock()
        .map_err(|err| err.to_string())?;

    if (*watcher_shutdown_state).is_some() {
        return Ok(());
    }

    *watcher_shutdown_state = Some(watcher_shutdown);

    spawn(move || {
        watcher.start_watch();
    });

    Ok(())
}

/// Stop listening for clipboard changes.
///
/// # Example
/// ```
/// use tauri_plugin_clipboard_x::stop_listening;
///
/// stop_listening().await?;
///```
#[command]
pub async fn stop_listening() -> Result<(), String> {
    let mut watcher_shutdown = get_clipboard_manager()
        .watcher
        .lock()
        .map_err(|err| err.to_string())?;

    if let Some(watcher_shutdown) = (*watcher_shutdown).take() {
        watcher_shutdown.stop();
    }

    *watcher_shutdown = None;

    Ok(())
}

/// Check if the clipboard contains plain text.
///
/// # Example
/// ```
/// use tauri_plugin_clipboard_x::has_text;
///
/// let has = has_text().await?;
/// println!("{has}"); // true
/// ```
#[command]
pub async fn has_text() -> Result<bool, String> {
    get_clipboard_manager().has(ContentFormat::Text)
}

/// Check if the clipboard contains rich text.
///
/// # Example
/// ```
/// use tauri_plugin_clipboard_x::has_rtf;
///
/// let has = has_rtf().await?;
/// println!("{has}"); // true
/// ```
#[command]
pub async fn has_rtf() -> Result<bool, String> {
    get_clipboard_manager().has(ContentFormat::Rtf)
}

/// Check if the clipboard contains html.
///
/// # Example
/// ```
/// use tauri_plugin_clipboard_x::has_html;
///
/// let has = has_html().await?;
/// println!("{has}"); // true
/// ```
#[command]
pub async fn has_html() -> Result<bool, String> {
    get_clipboard_manager().has(ContentFormat::Html)
}

/// Check if the clipboard contains image.
///
/// # Example
/// ```
/// use tauri_plugin_clipboard_x::has_image;
///
/// let has = has_image().await?;
/// println!("{has}"); // true
/// ```
#[command]
pub async fn has_image() -> Result<bool, String> {
    get_clipboard_manager().has(ContentFormat::Image)
}

/// Check if the clipboard contains files.
///
/// # Example
/// ```
/// use tauri_plugin_clipboard_x::has_files;
///
/// let has = has_files().await?;
/// println!("{has}"); // true
/// ```
#[command]
pub async fn has_files() -> Result<bool, String> {
    get_clipboard_manager().has(ContentFormat::Files)
}

/// Read the clipboard as plain text.
///
/// # Example
/// ```
/// use tauri_plugin_clipboard_x::read_text;
///
/// let text = read_text().await?;
/// println!("{read_text}"); // "Hello, world!"
/// ```
#[command]
pub async fn read_text() -> Result<String, String> {
    get_clipboard_manager()
        .ctx
        .lock()
        .map_err(|err| err.to_string())?
        .get_text()
        .map_err(|err| err.to_string())
}

/// Read the clipboard as rich text.
///
/// # Example
/// ```
/// use tauri_plugin_clipboard_x::read_rtf;
///
/// let rtf = read_rtf().await?;
/// println!("{rtf}"); // "{\\rtf1 Hello, world!}"
/// ```
#[command]
pub async fn read_rtf() -> Result<String, String> {
    get_clipboard_manager()
        .ctx
        .lock()
        .map_err(|err| err.to_string())?
        .get_rich_text()
        .map_err(|err| err.to_string())
}

/// Read the clipboard as html.
///
/// # Example
/// ```
/// use tauri_plugin_clipboard_x::read_html;
///
/// let html = read_html().await?;
/// println!("{html}"); // "<h1>Hello, world!</h1>"
/// ```
#[command]
pub async fn read_html() -> Result<String, String> {
    get_clipboard_manager()
        .ctx
        .lock()
        .map_err(|err| err.to_string())?
        .get_html()
        .map_err(|err| err.to_string())
}

/// Read the clipboard as an image.
///
/// # Example
/// ```
/// use tauri_plugin_clipboard_x::read_image;
///
/// let image = read_image().await?;
/// println!("{image}"); // ReadImage { path: "/path/to/xxx.png", size: 1024, width: 100, height: 100 }
/// ```
#[command]
pub async fn read_image<R: Runtime>(
    app_handle: AppHandle<R>,
    save_path: Option<PathBuf>,
) -> Result<ReadImage, String> {
    let default_save_path = get_default_save_image_path(app_handle).await?;
    let save_path = save_path.unwrap_or(default_save_path);

    create_dir_all(&save_path).map_err(|err| err.to_string())?;

    let image = get_clipboard_manager()
        .ctx
        .lock()
        .map_err(|err| err.to_string())?
        .get_image()
        .map_err(|err| err.to_string())?;

    let (width, height) = image.get_size();

    let dynamic_image = image.get_dynamic_image().map_err(|err| err.to_string())?;

    let bytes = dynamic_image.as_bytes();

    let mut hasher = DefaultHasher::new();

    bytes.hash(&mut hasher);

    let hash = hasher.finish();

    let path = save_path.join(format!("{hash}.png"));

    if let Some(path_str) = path.to_str() {
        image
            .save_to_path(path_str)
            .map_err(|err| err.to_string())?;

        let size = get_size(&path).unwrap_or(0);

        return Ok(ReadImage {
            path,
            size,
            width,
            height,
        });
    }

    Err("Failed to read clipboard image".to_string())
}

/// Read the clipboard as files.
///
/// # Example
/// ```
/// use tauri_plugin_clipboard_x::read_files;
///
/// let files = read_files().await?;
/// println!("{files}"); // ReadFile { paths: ["/path/to/xxx.txt", "/path/to/xxx"], size: 1024 }
/// ```
#[command]
pub async fn read_files() -> Result<ReadFile, String> {
    let mut paths = get_clipboard_manager()
        .ctx
        .lock()
        .map_err(|err| err.to_string())?
        .get_files()
        .map_err(|err| err.to_string())?;

    paths.iter_mut().for_each(|path| {
        *path = path.replace("file://", "");
    });

    let size = paths.iter().map(|path| get_size(path).unwrap_or(0)).sum();

    Ok(ReadFile { paths, size })
}

/// Write plain text to the clipboard.
///
/// # Example
/// ```
/// use tauri_plugin_clipboard_x::write_text;
///
/// write_text("Hello, world!").await?;
/// ```
#[command]
pub async fn write_text(text: String) -> Result<(), String> {
    get_clipboard_manager().write(vec![ClipboardContent::Text(text)])
}

/// Write rich text to the clipboard.
///
/// # Arguments
/// * `text` - The plain text.
/// * `rtf` - The rich text.
///
/// # Example
/// ```
/// use tauri_plugin_clipboard_x::write_rtf;
///
/// write_rtf("Hello, world!", "{\\rtf1 Hello, world!}").await?;
/// ```
#[command]
pub async fn write_rtf(text: String, rtf: String) -> Result<(), String> {
    let mut contents = vec![ClipboardContent::Rtf(rtf)];

    if cfg!(not(target_os = "macos")) {
        contents.push(ClipboardContent::Text(text))
    }

    get_clipboard_manager().write(contents)
}

/// Write html to the clipboard.
///
/// # Arguments
/// * `text` - The plain text.
/// * `html` - The html.
///
/// # Example
/// ```
/// use tauri_plugin_clipboard_x::write_html;
///
/// write_html("Hello, world!", "<h1>Hello, world!</h1>").await?;
/// ```
#[command]
pub async fn write_html(text: String, html: String) -> Result<(), String> {
    get_clipboard_manager().write(vec![
        ClipboardContent::Text(text),
        ClipboardContent::Html(html),
    ])
}

/// Write image to the clipboard.
///
/// # Example
/// ```
/// use tauri_plugin_clipboard_x::write_image;
///
/// write_image("/path/to/xxx.png").await?;
/// ```
#[command]
pub async fn write_image(image: String) -> Result<(), String> {
    let image = RustImageData::from_path(&image).map_err(|err| err.to_string())?;

    get_clipboard_manager().write(vec![ClipboardContent::Image(image)])
}

/// Write files to the clipboard.
///
/// # Example
/// ```
/// use tauri_plugin_clipboard_x::write_files;
///
/// write_files(vec!["/path/to/xxx.txt", "/path/to/xxx"]).await?;
/// ```
#[command]
pub async fn write_files(files: Vec<String>) -> Result<(), String> {
    get_clipboard_manager().write(vec![ClipboardContent::Files(files)])
}

/// Clear the clipboard.
///
/// # Example
/// ```
/// use tauri_plugin_clipboard_x::clear;
///
/// clear().await?;
/// ```
#[command]
pub async fn clear() -> Result<(), String> {
    get_clipboard_manager()
        .ctx
        .lock()
        .map_err(|err| err.to_string())?
        .clear()
        .map_err(|err| err.to_string())
}

/// Get the default save image path.
///
/// # Example
/// ```
/// use tauri_plugin_clipboard_x::get_default_save_image_path;
///
/// let path = get_default_save_image_path(app_handle).await?;
/// println!("{path}"); // "xx/tauri-plugin-clipboard-x/images"
/// ```
#[command]
pub async fn get_default_save_image_path<R: Runtime>(
    app_handle: AppHandle<R>,
) -> Result<PathBuf, String> {
    let path = app_handle
        .path()
        .app_data_dir()
        .map_err(|err| err.to_string())?
        .join("tauri-plugin-clipboard-x")
        .join("images");

    Ok(path)
}
