import { useEffect } from "react";
import {
  startListening,
  onClipboardChange,
  writeRTF,
} from "tauri-plugin-clipboard-x-api";

const App = () => {
  useEffect(() => {
    startListening();

    onClipboardChange((result) => {
      console.log("result", result);
    });

    setTimeout(() => {
      try {
        writeRTF("Hello, world!", "{\\rtf1 Hello, world!}");
      } catch (error) {
        console.log("error", error);
      }
    }, 3000);
  }, []);

  return <div>App</div>;
};

export default App;
