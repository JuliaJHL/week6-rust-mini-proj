# A Rust CLI tool: image resizing
This is a Rust CLI tool that can resizing images. It takes in three command line arguments: the path to the input image, the path to the output image, and the desired width of the output image. It calculates the new height of the output image based on the desired width and aspect ratio of the input image. After resizing, it saves the resized image to the output path.

## Project Setup
1. clone the repo:
```
https://github.com/JuliaJHL/week6-rust-mini-proj.git
```
2. cd into the project:
```
cd week6-rust-mini-proj
```
3. compile the project
```
cargo build --release
```
4. run the project
```
cargo run
```

## examples
When you run the project, it will prompt you the usage:
```
Welcome to Image Resizing!
Usage: cargo run <input_path> <output_path> <new_width>
```
There is a `Lenna.png` in the input folder with a size of 512x512. 

Resizing it to 1024x1024: `cargo run ./input/Lenna.png ./output/Lenna_up.png 1024` 

Resizing it to 256x256: `cargo run ./input/Lenna.png ./output/Lenna_down.png 256`

You can check all results in the `input` and `output` folders.

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
