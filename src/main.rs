extern crate image;

use image::*;

fn load_image(file_path: &str) -> DynamicImage {
    let buffer = image::open(file_path).unwrap();
    buffer
}

fn should_image_resize(width: u32) -> bool {
    width > 100
}

fn resize_img(image: &DynamicImage) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    let width = image.width();
    let height = image.height();

    if should_image_resize(width) {
        let resized = image::imageops::resize(
            image,
            width / 2,
            height / 2,
            image::imageops::FilterType::Nearest,
        );
        return resized;
    }
    image.to_rgba()
}

fn generate_ascii(image_buff: ImageBuffer<image::Rgba<u8>, Vec<u8>>) {
    let width = image_buff.width();
    let height = image_buff.height();

    let ascii_vec: Vec<char> =
        "`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$"
            .chars()
            .collect();
    for i in 0..height {
        println!("");
        for j in 0..width {
            let pixel = image_buff.get_pixel(j, i);
            let image::Rgba(data) = pixel;
            let average = (data[0] as i32 + data[1] as i32 + data[2] as i32) / 3;
            let ratio = average as f32 / 255.0;

            let mut ascii_eq = ratio * ascii_vec.len() as f32;

            if ascii_eq >= 65.0 {
                ascii_eq = 64.0;
            }

            print!("{}", ascii_vec[ascii_eq as usize])
        }
    }
}

fn main() {
    let image_to_convert = load_image("cat2.jpg");
    let resized_image = resize_img(&image_to_convert);
    generate_ascii(resized_image);
}
