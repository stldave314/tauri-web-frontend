# Tauri Web Frontend Bindings

This JavaScript library provides a convenient wrapper around the native Tauri commands exposed by the `tauri-web-frontend` application.

## Usage

Include this library in your web application that will be loaded by the Tauri frontend.

```javascript
import { greet, isTauriAvailable } from 'tauri-web-frontend-bindings';

if (isTauriAvailable()) {
    greet('World').then(response => {
        console.log(response); // "Hello, World! You've been greeted from Rust!"
    });
}
```

## Available Commands

- `greet(name: string): Promise<string>` - Sends a greeting request to the backend.
