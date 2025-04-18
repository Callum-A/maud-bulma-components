# Maud Component Library

A collection of reusable components for the [Maud](https://maud.lambda.xyz/) template framework in Rust, styled with [Bulma](https://bulma.io/) and enhanced with [Alpine.js](https://alpinejs.dev/).

## Features

- Pre-built, type-safe components for Maud
- Bulma-styled UI elements
- Interactive components powered by Alpine.js
- Fully customisable
- HTMX form support out of the box

## Installation

This crate is not yet published to crates.io. I recommend the following approach to allow maximum customisation:

- Clone the crate into your workspace/project
- Add as a dependency via a path e.g. `../maud-components`
- Use and customise as you see fit!

## Usage

```rust
use maud_component_library::components::*;

// Example component usage
html! {
    // Your components here
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.