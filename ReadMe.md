# 4D Raymarching in Rust

A 4D Raymarcher written in Rust: A project for learning about Rust by adapting some of the technologies from [4D Games](https://store.steampowered.com/app/2264190/4D_Games/).

As this project was created with the intention of learning rust, all code is for the CPU. This is by no means an efficient raymarcher as it does not use any shaders.

This project, like 4D Games, used Raymarching to render 4D shapes, and used Rotors of Geometric Algebra to manipulate these shapes without gimbal lock.

## Resources

 - [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)  
 - [The Ray Tracing Road to Rust](https://the-ray-tracing-road-to-rust.vercel.app/)  
 - [Game Development in Rust with SDL2](https://sunjay.dev/learn-game-dev/intro.html)

## Mathematics

The mathematcis module contains standard vector types: Float2, Float3, and Float4; and also tools of geometric algebra for rotation: Bivectors and Rotors for 3 and 4 dimensional rotation.

A series of unit tests for Rotor3 and Rotor4 are also included and can be run via `cargo test`

## Setup

To build and run the project you will need libsdl2 with ttf and image features. You can install these on Linux via:
```
sudo apt-get install libsdl2-dev
sudo apt-get install libsdl2-ttf-dev
sudo apt-get install libsdl2-image-dev
```

For more information or installation for other operating systems take a look at the [Rust-SDL2 Github](https://github.com/Rust-SDL2/rust-sdl2)

## TODO:
 - Multithreaded rendering for faster rendering and to learn about rust multithreading
