extern crate image;
extern crate color_quant;

use image::{GenericImageView, DynamicImage, Rgba};
use color_quant::{NeuQuant};

fn main() {
    let img = image::open("test.png").unwrap();
    let data = get_rgb(&img);
    // let data = vec![0; 40];
    let nq = NeuQuant::new(10, 256, &data);
    let indixes: Vec<u8> = data.chunks(4).map(|pix| nq.index_of(pix) as u8).collect();
    let color_map = nq.color_map_rgba();
    println!("{:?}", color_map);
    println!("{:?}", indixes);
}


fn get_rgb(img: &DynamicImage) -> Vec<u8> {
    let (imgx, imgy) = img.dimensions();
    // Create a new ImgBuf with width: imgx and height: imgy
    let mut data: Vec<u8> = vec![];
    // A redundant loop to demonstrate reading image data
    for x in 0..imgx {
        for y in 0..imgy {
            let Rgba(rgba) = img.get_pixel(x, y);
            data.push(rgba[0]);
            data.push(rgba[1]);
            data.push(rgba[2]);
            data.push(rgba[3]);
        }
    }

    data
}