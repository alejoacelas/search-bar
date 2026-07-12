<!--ai-->
# Search Bar

I want a clean, Raycast-style place to search my files now and add more commands later.

## Run it

```sh
npm install
npm run tauri dev
```

The first launch builds a local search index in the background. After that, type to search file names and text contents instantly. Click the folder name to change the search scope. Use ↑/↓ to move and Enter to open a result.

Press ⌘Space anywhere to show or hide Search Bar. First free that shortcut under System Settings → Keyboard → Keyboard Shortcuts → Spotlight by turning off “Show Spotlight search.”

## Build it

```sh
npm run tauri build
```

The React interface is in `src/`; native file search and open commands are in `src-tauri/src/lib.rs`. The UI talks to those commands through a small typed boundary so new actions can be added without coupling them to search.

See [How search works](docs/how-search-works.md) for matching, ranking, limitations, and comparisons. See [Search engine candidates](docs/search-engine-candidates.md) for the backend decision and upgrade path.
See [Universal search](docs/universal-search.md) for the plan to add email, repositories, messages, native actions, and optional multi-device sync.

The first connected-source slice can collect WhatsApp on this Mac now and move unchanged to an always-on Mac later. See [Collector setup](docs/collector-setup.md) and [Connector coverage](docs/connector-coverage.md).
<!--/ai-->
