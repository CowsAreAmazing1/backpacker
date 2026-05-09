fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Launch the native app using the library's helper.
        backpacker::run_native().expect("failed to start native app");
    }

    #[cfg(target_arch = "wasm32")]
    {
        // For wasm builds, the web entrypoint is provided by the library (`WebHandle`).
    }
}
