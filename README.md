# Task Manager with Rust and Actix-web

A simple task management web application built with Rust,
demonstrating the use of enums, Actix-web framework,
and Askama templating.

## Features

- Display tasks with different categories (Work, Personal, Hobby)
- Visual distinction between task categories using CSS styling
- Server-side rendering with Askama templates
- Type-safe task categorization using Rust enums

## Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager)

## Installation

1. Clone the repository:

```bash


git clone https://github.com/warathepj/rust-enum.git
cd rust-enum
```

2. Build and run the project:

```bash
cargo run
```

3. Open your web browser and visit:

```
http://localhost:8080

```

## Project Structure

- `src/main.rs` - Main application code with task and category definitions
- `templates/index.html` - HTML template with styling
- `Cargo.toml` - Project dependencies and configuration

## Dependencies

- actix-web (4.0) - Web framework
- askama (0.12) - Templating engine
- serde (1.0) - Serialization framework

## License

This project is licensed under the MIT License - see the LICENSE file for details.

TODO:
