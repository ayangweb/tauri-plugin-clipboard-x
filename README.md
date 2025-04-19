# tauri-plugin-clipboard-x

> This plugin only works on tauri v2, if you need the v1 plugin, feel free to submit a PR!

Supports clipboard change listening and enables reading and writing of various clipboard formats (plain text, rich text, html, images, and files).

## Platform Support

| Platform | Supported |
| -------- | --------- |
| Windows  | ✅        |
| macOS    | ✅        |
| Linux    | ✅        |
| Android  | ⏳        |
| iOS      | ⏳        |

## Install

```shell
cargo add tauri-plugin-clipboard-x
```

You can install the JavaScript Guest bindings using your preferred JavaScript package manager:

```shell
pnpm add tauri-plugin-clipboard-x-api
```

## Usage

`src-tauri/src/lib.rs`

```diff
pub fn run() {
    tauri::Builder::default()
+       .plugin(tauri_plugin_clipboard_x::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

`src-tauri/capabilities/default.json`

```diff
{
    ...
    "permissions": [
        ...
+       "clipboard-x:default"
    ]
}
```

Afterwards all the plugin's APIs are available through the JavaScript guest bindings:

```ts
import {
  startListening,
  onClipboardChange,
} from "tauri-plugin-clipboard-x-api";

await startListening();

const unlisten = await onClipboardChange((result) => {
  console.log(result);
});
```

## Methods

| Method                    | Description                                    |
| ------------------------- | ---------------------------------------------- |
| `startListening`          | Start listening for clipboard changes.         |
| `stopListening`           | Stop listening for clipboard changes.          |
| `hasText`                 | Check if the clipboard contains plain text.    |
| `hasRTF`                  | Check if the clipboard contains rich text.     |
| `hasHTML`                 | Check if the clipboard contains html.          |
| `hasImage`                | Check if the clipboard contains an image.      |
| `hasFiles`                | Check if the clipboard contains files.         |
| `readText`                | Read the clipboard as plain text.              |
| `readRTF`                 | Read the clipboard as rich text.               |
| `readHTML`                | Read the clipboard as html.                    |
| `readImage`               | Read the clipboard as an image.                |
| `readFiles`               | Read the clipboard as files.                   |
| `writeText`               | Write plain text to the clipboard.             |
| `writeRTF`                | Write rich text to the clipboard.              |
| `writeHTML`               | Write html to the clipboard.                   |
| `writeImage`              | Write an image to the clipboard.               |
| `writeFiles`              | Write files to the clipboard.                  |
| `clear`                   | Clear the clipboard.                           |
| `getDefaultSaveImagePath` | Get the default save image path.               |
| `readClipboard`           | Read all available content from the clipboard. |
| `onClipboardChange`       | Listen to clipboard changes.                   |

## Example

```shell
git clone https://github.com/ayangweb/tauri-plugin-clipboard-x.git
```

```shell
pnpm install

pnpm build

cd examples/tauri-app

pnpm install

pnpm tauri dev
```

## Thanks

- Use [clipboard-rs](https://github.com/ChurchTao/clipboard-rs) for clipboard operations across different platforms.

## Who's Use It

- [EcoPaste](https://github.com/EcoPasteHub/EcoPaste) - Open source cross-platform clipboard management tool.
