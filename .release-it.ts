import type { Config } from "release-it";

export default {
  git: {
    commitMessage: "tauri-plugin-clipboard-x-api v${version}",
    tagName: "v${version}",
  },
  npm: {
    publish: true,
  },
} satisfies Config;
