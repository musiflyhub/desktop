# Musifly Desktop

![Musifly App Screenshot](https://providercontent.valgix.com/img/musifly/app_screen.png)

Musifly Desktop is the official native client for [Musifly](https://musifly.net), the premier destination for listening to music for free. This application brings the full Musifly experience to your desktop with enhanced native features and a seamless interface.

## Features

- **Native Desktop Experience**: A fast, lightweight wrapper for the Musifly platform.
- **Close to Tray**: Keep the music playing by minimizing the app to your system tray.
- **Auto-Updates**: Stay up to date with the latest features and security patches automatically via GitHub.
- **Splash Screen**: A sleek loading experience that checks for updates on startup.

## Technologies Used

This project is built using modern, high-performance technologies:

- **[Tauri v2](https://tauri.app/)**: The secure, lightweight framework for building cross-platform desktop apps.
- **[React](https://reactjs.org/)**: A powerful JavaScript library for building user interfaces.
- **[TypeScript](https://www.typescriptlang.org/)**: Typed JavaScript for better developer productivity and code quality.
- **[Rust](https://www.rust-lang.org/)**: The performance-focused backend language that powers the native features.
- **[Bun](https://bun.sh/)**: A fast all-in-one JavaScript runtime and package manager.
- **[Vite](https://vitejs.dev/)**: A blazing fast frontend build tool.
- **[Tailwind CSS](https://tailwindcss.com/)**: A utility-first CSS framework for rapid UI development.

## Development

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Bun](https://bun.sh/)

### Running Locally

```bash
bun install
bun run tauri dev
```

### Building for Production

```bash
bun run tauri build
```

## License

This project is licensed under the [Apache License 2.0](LICENSE).
