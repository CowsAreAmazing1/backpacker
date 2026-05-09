@echo off
setlocal enabledelayedexpansion

echo Building WASM library...
REM Force WASM build to local target/ directory only for this build
set CARGO_TARGET_DIR=target
cargo build --lib --target wasm32-unknown-unknown --release
if errorlevel 1 exit /b 1
REM Clear the override so subsequent cargo commands use the default/global target dir
set CARGO_TARGET_DIR=

echo.
echo Generating WASM bindings...

set WASM_FILE=target\wasm32-unknown-unknown\release\backpacker.wasm

if exist "!WASM_FILE!" (
    echo Found WASM at !WASM_FILE!
) else (
    echo Error: Could not find backpacker.wasm at !WASM_FILE!
    exit /b 1
)

wasm-bindgen "!WASM_FILE!" --out-dir pkg --target web --out-name backpacker
if errorlevel 1 exit /b 1

echo.
echo Starting server...
cargo run --bin server