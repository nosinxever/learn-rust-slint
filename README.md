# Rust Slint Counter App

A simple desktop application built with Rust and Slint UI framework that demonstrates basic UI interactions with a counter.

## Features

- Clean and modern UI design
- Interactive button with visual feedback
- Click counter functionality
- Custom window icon
- Responsive layout with proper spacing and alignment

## Prerequisites

Before running this application, make sure you have the following installed:
- Rust (latest stable version)
- Cargo (comes with Rust)
- A C++ build environment (for Windows: MSVC build tools)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/nosinxever/learn-rust-slint.git
cd learn-rust-slint
```

2. Build and run the application:
```bash
cargo run
```

## Project Structure

```
learn/
├── src/
│   ├── main.rs           # Application entry point
│   └── ui/
│       ├── mod.rs        # UI module declarations
│       └── main_window.rs # Main window UI component
├── assets/
│   ├── app_icon.png      # Application icon
│   ├── windows/          # Windows-specific assets
│   └── macos/            # macOS-specific assets
├── Cargo.toml            # Project dependencies
└── README.md            # Project documentation
```

## Dependencies

This project uses the following dependencies:
- `slint` - For building the user interface

## Development

The application is built using:
- Rust - For the application logic
- Slint - For declarative UI design

## Contributing

Feel free to contribute to this project by:
1. Forking the repository
2. Creating your feature branch (`git checkout -b feature/AmazingFeature`)
3. Committing your changes (`git commit -m 'Add some AmazingFeature'`)
4. Pushing to the branch (`git push origin feature/AmazingFeature`)
5. Opening a Pull Request

## License

This project is open source and available under the MIT License.
