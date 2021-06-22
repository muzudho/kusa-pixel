# Settings

毎秒更新したかったがやり方が分からなかったので、ウィンドウ上でマウスを動かし続けると 更新される仕組みに（＾～＾）  

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
    "canvas_margin_top": 10.0,
    "canvas_margin_right": 20.0,
    "canvas_margin_bottom": 10.0,
    "canvas_margin_left": 20.0
}
```

👆 Leave a gap of a few pixels between the border and the image  

```json
{
    "canvas_cell_size": 10.0
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
    "paint_tool": "Pen",
    "paint_nib": "Circle"
}
```

👆 Now it's just a pen. Draw a point  

```json
{
    "paint_color": {
        "r": 128,
        "g": 128,
        "b": 255,
        "a": 255
    }
}
```

👆 Colors to be painted, 0-255. r is red, g is green, b is blue. rgb is 0 for black. a is 255 for opaque

```json
{
    "paint_thickness": 2.0
}
```

👆 Line thickness
