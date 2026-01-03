/**
 * Bindings for Tauri Web Frontend
 * 
 * This library exposes the native Tauri commands to the loaded frontend.
 */

// Detect Tauri internals
// In Tauri v2 with Global Tauri enabled, it's typically window.__TAURI__.core.invoke
// In older versions or compat mode, it might be window.__TAURI__.invoke
const invoke = window.__TAURI__?.core?.invoke || window.__TAURI__?.invoke;

/**
 * Checks if the Tauri API is available in the current environment.
 * @returns {boolean} True if running within Tauri with APIs exposed.
 */
export function isTauriAvailable() {
    return !!invoke;
}

/**
 * Greets a person via the Rust backend.
 * This is a sample command demonstrating the native binding.
 * 
 * @param {string} name - The name to greet.
 * @returns {Promise<string>} - The greeting message from Rust.
 */
export async function greet(name) {
    if (!invoke) {
        console.warn("Tauri API not found. Returning mock response.");
        return `Hello, ${name}! (Mock - Tauri not detected)`;
    }
    try {
        return await invoke('greet', { name });
    } catch (e) {
        console.error("Failed to invoke greet:", e);
        throw e;
    }
}
