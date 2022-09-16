use image;
use tobj;

mod vec;
mod shapes {
    pub mod shape;
    pub mod sphere;
}

fn main() {
    let img_size = (128, 128);
    let mut img = image::RgbImage::new(img_size.0, img_size.1);
    let radius: i32 = 32;
    let bias: (i32, i32) = (8, 24);

    for x in 0..img.width() as i32 {
        for y in 0..img.height() as i32 {
            if (x - bias.0).pow(2) + (y - bias.1).pow(2) <= radius.pow(2) {
                img.put_pixel(x as u32, y as u32, image::Rgb([0, 255, 0]));
            } else {
                img.put_pixel(x as u32, y as u32, image::Rgb([0, 0, 255]));
            }
        }
    }

    let path = "./img.png";
    img.save_with_format(path, image::ImageFormat::Png);

    println!("Hello, world!");
}
