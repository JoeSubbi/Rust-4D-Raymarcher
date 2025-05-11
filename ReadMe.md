# 4D Raymarching in Rust

A 4D Raymarcher written in Rust: A project with the purpose of learning Rust by adapting some of the technologies from [4D Games](https://store.steampowered.com/app/2264190/4D_Games/).

This project does not make use of shaders as this project was created with the intention of learning rust. The raymarcher uses multi-threading, but is of course not an efficient raymarcher without the GPU, reaching up to 30fps on an AMD Ryzen 7.

This project, like 4D Games, uses Rotors of Geometric Algebra to rotate these shapes without gimbal lock.

## Command Line Arguments

 - `--aa` Anti Aliasing. Performance intensive, but less pixilated renders.
 - `--d` Render a 3D scene instead.

## Mathematics Module

The project includes a mathematcis module which contains common Vector types: Float2, Float3, and Float4; and also tools of geometric algebra for rotation: Bivectors and Rotors for 3 and 4 dimensions

There are also unit tests for Rotor3 and Rotor4 and can be run via `cargo test`

## SDL2

To build and run the project you will need `libsdl2` with `ttf` and `image` features. These can be installed on Linux via:
```
sudo apt-get install libsdl2-dev
sudo apt-get install libsdl2-ttf-dev
sudo apt-get install libsdl2-image-dev
```

For more information or how to installation SLD2 for other operating systems take a look at the [Rust-SDL2 Github](https://github.com/Rust-SDL2/rust-sdl2)

## Learning Resources

As this is a learning project here are the resources I used:

 - [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)  
 - [The Ray Tracing Road to Rust](https://the-ray-tracing-road-to-rust.vercel.app/) a verion of Ray Tracing in One Weekend but for Rust  
 - [Game Development in Rust with SDL2](https://sunjay.dev/learn-game-dev/intro.html) for the set up of SDL2 for Rust
