# Tauri Web Frontend

A flexible Tauri-based web frontend wrapper that can load any URL specified via command-line arguments. It exposes native system bindings to the loaded page via a JavaScript library.

## Features

- **Load Any URL**: Specify the target URL at runtime using the `--url` argument.
- **Native Bindings**: JavaScript library provided to interact with the Rust backend.
- **Cross-Platform**: Builds for Windows, macOS, and Linux.

## Getting Started

### Prerequisites

- Rust (Cargo)
- Node.js (npm/pnpm/yarn)
- System dependencies for Tauri (see [Tauri Docs](https://tauri.app/start/prerequisites/))

### Installation

Clone the repository and install dependencies:

```bash
git clone <repository_url>
cd tauri-web-frontend
npm install
```

### Running the App

Run the application in development mode:

```bash
npm run tauri dev
```

To load a specific URL:

```bash
npm run tauri dev -- -- --url https://google.com
```
*Note: The extra `--` are needed to pass arguments through the npm script to the binary.*

### Building

Build the application for production:

```bash
npm run tauri build
```

The output binary will be in `src-tauri/target/release/`.

## usage with Native Bindings

The application exposes native commands (like `greet`) to the loaded page. To use them, include the provided JavaScript library in your web project.

### JavaScript Library

The bindings library is located in `js-lib/`. You can publish this package or include it directly in your frontend project.


## JavaScript API

The web application loaded by Tauri has access to the `window.__TAURI__` object, exposing the full Tauri API (if enabled).

In addition, we provide a wrapper library `tauri-web-frontend-bindings` with the following helper functions:

### `greet(name: string): Promise<string>`
Sends a greeting request to the Rust backend.
- **Example**: `match greet('Dave') { Ok(msg) => console.log(msg) }`

### `isTauriAvailable(): boolean`
Checks if the application is running within the Tauri context.

### Official API
For the complete list of available system APIs (File System, Dialog, standard Window controls, etc.), please refer to the [Tauri v2 API Documentation](https://v2.tauri.app/reference/).


## Configuration

The application accepts several command-line arguments to customize its behavior and appearance.

### Window Management

| Argument | Description | Example |
| :--- | :--- | :--- |
| `--url <URL>` | The URL to load on startup. | `--url https://google.com` |
| `--width <pixels>` | Set the initial window width. | `--width 1024` |
| `--height <pixels>` | Set the initial window height. | `--height 768` |
| `--fullscreen` | Start the application in fullscreen mode. | `--fullscreen` |
| `--minimized` | Start the application minimized. | `--minimized` |
| `--no-resizable` | Disable window resizing by the user. | `--no-resizable` |
| `--frameless` | Remove the window frame and title bar. | `--frameless` |

### Security & Restrictions

| Argument | Description | Example |
| :--- | :--- | :--- |
| `--allowed-domains <list>` | Comma-separated list of domains allowed for navigation. Navigating to other domains will be blocked. | `--allowed-domains google.com,github.com` |
| `--api-domains <list>` | Comma-separated list of domains allowed to use the native API (bindings). | `--api-domains myapp.com` |

