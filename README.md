# CRusQ

A simple desktop app that generates QR codes from clipboard content.

## Features

- Auto-generate QR code from clipboard on launch
- Native blur effects (Windows Acrylic/Mica, macOS Vibrancy)
- Frameless draggable window
- High error correction QR codes

## Tech Stack

- Tauri 2.0
- Rust
- Vanilla JS

## Setup

```bash
npm install
npm run tauri dev
```

## Build

```bash
npm run tauri build
```

Binary will be in `src-tauri/target/release/`
