# RustyFilters

RustyFilters is an image processing tool written in Rust that applies various filters and effects to images.

## Features

- Add grain effect
- Enhance colors
- Apply subtle glow
- Sharpen images

## Installation

1. Make sure you have Rust installed on your system. If not, you can install it from [https://www.rust-lang.org/](https://www.rust-lang.org/).

2. Clone this repository:
   ```
   git clone https://github.com/Cliffy57/RustyFilters.git
   cd rustyfilters
   ```

3. Build the project:
   ```
   cargo build --release
   ```

## Usage

1. Place your input image in the `src` directory and name it `input.png`.

2. Run the program:
   ```
   cargo run --release
   ```
   Debug mode:
   ```
   cargo build --release
   set RUST_LOG=info  
   .\target\release\rust_image_filter.exe
   ```

3. The processed image will be saved as `output.png` in the `src` directory.

## Documentation

This project uses Rust's built-in documentation system. To generate and view the documentation:

1. Generate the documentation:
   ```
   cargo doc --open
   ```

2. This will build the documentation and open it in your default web browser.

To update the documentation, simply run the `cargo doc` command again after making changes to your code.

## API Reference

### `apply_filter`

Applies various filters and effects to an input image and saves the result.

### `add_grain`

Adds a grain effect to the image by introducing random noise.

### `enhance_colors`

Enhances colors using a subtle technique.

### `add_glow`

Adds a very subtle glow effect to the image.

### `sharpen`

Sharpens the image using a simple convolution kernel.

For more detailed information about each function, including parameters and return types, please refer to the generated documentation.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.