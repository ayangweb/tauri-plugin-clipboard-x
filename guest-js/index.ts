import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export const COMMAND = {
  START_LISTENING: "plugin:clipboard-x|start_listening",
  STOP_LISTENING: "plugin:clipboard-x|stop_listening",
  HAS_TEXT: "plugin:clipboard-x|has_text",
  HAS_RTF: "plugin:clipboard-x|has_rtf",
  HAS_HTML: "plugin:clipboard-x|has_html",
  HAS_IMAGE: "plugin:clipboard-x|has_image",
  HAS_FILES: "plugin:clipboard-x|has_files",
  READ_TEXT: "plugin:clipboard-x|read_text",
  READ_RTF: "plugin:clipboard-x|read_rtf",
  READ_HTML: "plugin:clipboard-x|read_html",
  READ_IMAGE: "plugin:clipboard-x|read_image",
  READ_FILES: "plugin:clipboard-x|read_files",
  WRITE_TEXT: "plugin:clipboard-x|write_text",
  WRITE_RTF: "plugin:clipboard-x|write_rtf",
  WRITE_HTML: "plugin:clipboard-x|write_html",
  WRITE_IMAGE: "plugin:clipboard-x|write_image",
  WRITE_FILES: "plugin:clipboard-x|write_files",
  CLEAR: "plugin:clipboard-x|clear",
  GET_DEFAULT_SAVE_IMAGE_PATH: "plugin:clipboard-x|get_default_save_image_path",
  CLIPBOARD_CHANGED: "plugin:clipboard-x://clipboard_changed",
};

export type ClipboardType = "text" | "rtf" | "html" | "image" | "files";

export interface ReadImage {
  // The path of the image.
  path: string;
  // The size of the image in bytes.
  size: number;
  // The width of the image in pixels.
  width: number;
  // The height of the image in pixels.
  height: number;
}

export interface ReadFile {
  // The paths of the files.
  paths: string[];
  // The size of the files in bytes.
  size: number;
}

interface ReadClipboardItem<T = string> {
  // The type of the clipboard content.
  type: ClipboardType;
  // The value of the clipboard content.
  value: T;
  // The size or length of the clipboard content.
  count: number;
}

export interface ReadClipboard {
  text?: ReadClipboardItem;
  rtf?: ReadClipboardItem;
  html?: ReadClipboardItem;
  image?: ReadClipboardItem & Omit<ReadImage, "path" | "size">;
  files?: ReadClipboardItem<string[]>;
}

export type ClipboardChangeCallback = (result: ReadClipboard) => void;

export interface ClipboardChangeOptions {
  // A hook that will be called before reading clipboard content.
  beforeRead?: () => void;
}

/**
 * Start listening for clipboard changes.
 *
 * @example
 * ```
 * import { startListening } from 'tauri-plugin-clipboard-x-api';
 *
 * await startListening();
 * ```
 */
export const startListening = () => {
  return invoke(COMMAND.START_LISTENING);
};

/**
 * Stop listening for clipboard changes.
 *
 * @example
 * ```
 * import { stopListening } from 'tauri-plugin-clipboard-x-api';
 *
 * await stopListening();
 * ```
 */
export const stopListening = () => {
  return invoke(COMMAND.STOP_LISTENING);
};

/**
 * Check if the clipboard contains plain text.
 *
 * @example
 * ```
 * import { hasText } from 'tauri-plugin-clipboard-x-api';
 *
 * const has = await hasText();
 * console.log(has); // true
 * ```
 */
export const hasText = () => {
  return invoke<boolean>(COMMAND.HAS_TEXT);
};

/**
 * Check if the clipboard contains rich text.
 *
 * @example
 * ```
 * import { hasRTF } from 'tauri-plugin-clipboard-x-api';
 *
 * const has = await hasRTF();
 * console.log(has); // true
 * ```
 */
export const hasRTF = () => {
  return invoke<boolean>(COMMAND.HAS_RTF);
};

/**
 * Check if the clipboard contains html.
 *
 * @example
 * ```
 * import { hasHTML } from 'tauri-plugin-clipboard-x-api';
 *
 * const has = await hasHTML();
 * console.log(has); // true
 * ```
 */
export const hasHTML = () => {
  return invoke<boolean>(COMMAND.HAS_HTML);
};

/**
 * Check if the clipboard contains an image.
 *
 * @example
 * ```
 * import { hasImage } from 'tauri-plugin-clipboard-x-api';
 *
 * const has = await hasImage();
 * console.log(has); // true
 * ```
 */
export const hasImage = () => {
  return invoke<boolean>(COMMAND.HAS_IMAGE);
};

/**
 * Check if the clipboard contains files.
 *
 * @example
 * ```
 * import { hasFiles } from 'tauri-plugin-clipboard-x-api';
 *
 * const has = await hasFiles();
 * console.log(has); // true
 * ```
 */
export const hasFiles = () => {
  return invoke<boolean>(COMMAND.HAS_FILES);
};

/**
 * Read the clipboard as plain text.
 *
 * @example
 * ```
 * import { readText } from 'tauri-plugin-clipboard-x-api';
 *
 * const text = await readText();
 * console.log(text); // "Hello, world!"
 * ```
 */
export const readText = () => {
  return invoke<string>(COMMAND.READ_TEXT);
};

/**
 * Read the clipboard as rich text.
 *
 * @example
 * ```
 * import { readRTF } from 'tauri-plugin-clipboard-x-api';
 *
 * const rtf = await readRTF();
 * console.log(rtf); // "{\\rtf1 Hello, world!}"
 * ```
 */
export const readRTF = () => {
  return invoke<string>(COMMAND.READ_RTF);
};

/**
 * Read the clipboard as html.
 *
 * @example
 * ```
 * import { readHTML } from 'tauri-plugin-clipboard-x-api';
 *
 * const html = await readHTML();
 * console.log(html); // "<h1>Hello, world!</h1>"
 * ```
 */
export const readHTML = () => {
  return invoke<string>(COMMAND.READ_HTML);
};

/**
 * Read the clipboard as an image.
 *
 * @example
 * ```
 * import { readImage } from 'tauri-plugin-clipboard-x-api';
 *
 * const image = await readImage();
 * console.log(image); // { path: "/path/to/xxx.png", size: 1024, width: 100, height: 100 }
 * ```
 */
export const readImage = () => {
  return invoke<ReadImage>(COMMAND.READ_IMAGE);
};

/**
 * Read the clipboard as files.
 *
 * @example
 * ```
 * import { readFiles } from 'tauri-plugin-clipboard-x-api';
 *
 * const files = await readFiles();
 * console.log(files); // { paths: ["/path/to/xxx.txt", "/path/to/xxx"], size: 1024 }
 * ```
 */
export const readFiles = () => {
  return invoke<ReadFile>(COMMAND.READ_FILES);
};

/**
 * Write plain text to the clipboard.
 *
 * @example
 * ```
 * import { writeText } from 'tauri-plugin-clipboard-x-api';
 *
 * await writeText("Hello, world!");
 * ```
 */
export const writeText = (text: string) => {
  return invoke(COMMAND.WRITE_TEXT, { text });
};

/**
 * Write rich text to the clipboard.
 *
 * @param text The plain text.
 * @param rtf The rich text.
 *
 * @example
 * ```
 * import { writeRTF } from 'tauri-plugin-clipboard-x-api';
 *
 * await writeRTF("Hello, world!", "{\\rtf1 Hello, world!}");
 * ```
 */
export const writeRTF = (text: string, rtf: string) => {
  return invoke(COMMAND.WRITE_RTF, { text, rtf });
};

/**
 * Write html to the clipboard.
 *
 * @param text The plain text.
 * @param html The html.
 *
 * @example
 * ```
 * import { writeHTML } from 'tauri-plugin-clipboard-x-api';
 *
 * await writeHTML("Hello, world!", "<h1>Hello, world!</h1>");
 * ```
 */
export const writeHTML = (text: string, html: string) => {
  return invoke(COMMAND.WRITE_HTML, { text, html });
};

/**
 * Write an image to the clipboard.
 *
 * @example
 * ```
 * import { writeImage } from 'tauri-plugin-clipboard-x-api';
 *
 * await writeImage("/path/to/xxx.png");
 * ```
 */
export const writeImage = (image: string) => {
  return invoke(COMMAND.WRITE_IMAGE, { image });
};

/**
 * Write files to the clipboard.
 *
 * @example
 * ```
 * import { writeFiles } from 'tauri-plugin-clipboard-x-api';
 *
 * await writeFiles(["/path/to/xxx.txt", "/path/to/xxx"]);
 * ```
 */
export const writeFiles = (files: string[]) => {
  return invoke(COMMAND.WRITE_FILES, { files });
};

/**
 * Clear the clipboard.
 *
 * @example
 * ```
 * import { clear } from 'tauri-plugin-clipboard-x-api';
 *
 * await clear();
 * ```
 */
export const clear = () => {
  return invoke(COMMAND.CLEAR);
};

/**
 * Get the default save image path.
 *
 * @example
 * ```
 * import { getDefaultSaveImagePath } from 'tauri-plugin-clipboard-x-api';
 *
 * const path = await getDefaultSaveImagePath();
 * console.log(path); // "xx/tauri-plugin-clipboard-x/images"
 * ```
 */
export const getDefaultSaveImagePath = () => {
  return invoke<string>(COMMAND.GET_DEFAULT_SAVE_IMAGE_PATH);
};

/**
 * Read all available content from the clipboard.
 *
 * @example
 * ```
 * import { readClipboard } from 'tauri-plugin-clipboard-x-api';
 *
 * const result = await readClipboard();
 * ```
 *
@returns Promise<{@link ReadClipboard}>
 */
export const readClipboard = async () => {
  const result: ReadClipboard = {};

  if (await hasText()) {
    const text = await readText();

    result.text = {
      type: "text",
      value: text,
      count: text.length,
    };
  }

  if (await hasRTF()) {
    const rtf = await readRTF();

    result.rtf = {
      type: "rtf",
      value: rtf,
      count: result.text?.count ?? rtf.length,
    };
  }

  if (await hasHTML()) {
    const html = await readHTML();

    result.html = {
      type: "html",
      value: html,
      count: result.text?.count ?? html.length,
    };
  }

  if (await hasImage()) {
    const { path, size, ...rest } = await readImage();

    result.image = {
      type: "image",
      value: path,
      count: size,
      ...rest,
    };
  }

  if (await hasFiles()) {
    const { paths, size } = await readFiles();

    result.files = {
      type: "files",
      value: paths,
      count: size,
    };
  }

  return result;
};

/**
 * Listen to clipboard changes.
 *
 * @param cb The callback function that will be called when clipboard changes.
 * @param options.beforeRead A hook that will be called before reading clipboard content.
 *
 * @example
 * ```
 * import { startListening, onClipboardChange } from 'tauri-plugin-clipboard-x-api';
 *
 * await startListening();
 *
 * const unlisten = await onClipboardChange((result) => {
 *   console.log(result);
 * });
 * ```
 */
export const onClipboardChange = (
  cb: ClipboardChangeCallback,
  options: ClipboardChangeOptions = {}
) => {
  const { beforeRead } = options;

  return listen(COMMAND.CLIPBOARD_CHANGED, async () => {
    beforeRead?.();

    const result = await readClipboard();

    cb(result);
  });
};
