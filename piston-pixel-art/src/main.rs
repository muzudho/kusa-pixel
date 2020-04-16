extern crate image;

mod res;

use crate::res::image::Frame;
use crate::res::logic::image_writer::*;
use crate::res::window::window::*;

fn main() {
    // サイズ指定で☆（＾～＾）
    {
        let frame = Frame::new(32, 64);
        write_frame(&frame, "output/32x64.png");
    }

    show_window("output/test.png");
}
