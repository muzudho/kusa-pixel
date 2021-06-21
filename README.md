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

```json
{
    "image_height": 96
}
```

👆 Height of the image file in pixels  

```json
{
    "canvas_zoom": 1.0
}
```

👆 Zoom in/out when drawing. It does not affect the image to be saved  

```json
{
    "canvas_margin_x": 20.0,
    "canvas_margin_y": 10.0
}
```

👆 Leave a gap of a few pixels between the left (top) border and the image  

```json
{
    "canvas_dot_width": 10.0,
    "canvas_dot_height": 10.0
}
```

👆 The size of a pixel in the drawing window. It does not affect the image to be saved  

```json
{
    "canvas_grid_thickness": 0.5,
    "canvas_grid_color": [
        0.0,
        0.0,
        0.0,
        1.0
    ]
}
```

👆 The thickness of the grid lines and the color (RGBA)  

```json
{
    "paint_tool": "pen"
}
```

👆 Now it's just a pen. Draw a point  
