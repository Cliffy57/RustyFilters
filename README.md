# RustyFilters

![RustyFilters Logo](RustyFilters.png)

RustyFilters is an image processing tool written in Rust that applies various filters and effects to images.

## Features

- Add grain effect
- Enhance colors
- Apply subtle glow
- Sharpen images

## Prerequisites

Before running this project, ensure you have the following installed:

1. **Rust**: You can install Rust from [https://www.rust-lang.org/](https://www.rust-lang.org/).
2. **ffmpeg**: This project uses `ffmpeg` to optimize the size of the output images. Follow the instructions below to install `ffmpeg` on your system.

### Installing ffmpeg

#### Windows

1. Download the `ffmpeg` executable from the [official website](https://ffmpeg.org/download.html).
2. Extract the downloaded archive.
3. Add the `bin` directory containing `ffmpeg.exe` to your system's `PATH` environment variable.

#### Linux

Install `ffmpeg` using your package manager. For example, on Ubuntu, you can run:

```sh
sudo apt-get update
sudo apt-get install ffmpeg# RustyFilters
```
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

## Reference

This project provides the following functions:

- `add_grain`: Adds a grain effect to the image.
- `enhance_colors`: Enhances the colors of the image.
- `apply_glow`: Applies a subtle glow effect to the image.
- `sharpen`: Sharpens the image.
- `save_image`: Saves the image to the specified file path.
- `load_image`: Loads an image from the specified file path.
- `optimize_image`: Optimizes the size of the image using ffmpeg.
- `process_image`: Processes the image using the specified filters.
- `main`: The main function that runs the program.
- `get_image_path`: Gets the path to the input image.
- `get_output_path`: Gets the path to the output image.
- `get_image`: Gets the image from the specified file path.
- `get_image_dimensions`: Gets the dimensions of the image.
- `get_image_data`: Gets the image data.
- `set_image_data`: Sets the image data.
- `get_pixel`: Gets the pixel at the specified coordinates.
- `set_pixel`: Sets the pixel at the specified coordinates.
- `clamp`: Clamps the value to the specified range.
- `apply_filter`: Applies the specified filter to the image.
- `apply_grain`: Applies the grain effect to the image.
- `apply_color_enhancement`: Enhances the colors of the image.
- `apply_glow_effect`: Applies the glow effect to the image.
- `apply_sharpening`: Sharpens the image.
- `apply_convolution`: Applies a convolution filter to the image.
- `convolve`: Convolves the image with the specified kernel.

For more detailed information about each function, including parameters and return types, please refer to the generated documentation.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgements

- [image](https://crates.io/crates/image) - A basic image processing library.

- [imageproc](https://crates.io/crates/imageproc) - A library for image processing.

- [ffmpeg](https://ffmpeg.org/) - A complete, cross-platform solution to record, convert, and stream audio and video.

- [Rust](https://www.rust-lang.org/) - A language empowering everyone to build reliable and efficient software.

- [Cliffy57](https://github.com/Cliffy57) - The author of this project.
