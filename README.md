# Kusa pixel

This is a pixel art painting software.  
kusa is Japanese internet slang for laughing at

`m9（＾～＾）wwwww 草（＾▽＾）`  

This program will be distributed at MIT LICENSE.  

It uses the libraries of Piston, a game engine in the Rust language.  
[PistonDevelopers](https://github.com/PistonDevelopers)  

It is intended to be developed and run using Visual Studio Code on Windows.  

## Features or concepts

* It is a pain in the ass to create a GUI,  
so please edit the configuration file for pen thickness and other settings.
* It's a hassle, so the configuration file is not very structured and flat.

## Build

```shell
// Updating the Rust language
rustup update

// Creating an executable file
cargo build
```

## Run

```shell
cargo run
```

## Settings

settings.json:  

```json
{
    "image_file": "./work/image.png"
}
```

👆 Set the path to the image file  

```json
{
    "image_width": 64
}
```

👆 Width of the image file in pixels  
