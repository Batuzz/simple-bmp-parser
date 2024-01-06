use image::{ImageBuffer, Rgb};
use rand::Rng;
use std::fs::read;

struct Size(u32, u32);

fn generate_random_image(size: &Size) {
    let mut img = ImageBuffer::new(size.0, size.1);
    let mut rng = rand::thread_rng();

    for (_, _, pixel) in img.enumerate_pixels_mut() {
        let r: u8 = rng.gen();
        let g: u8 = rng.gen();
        let b: u8 = rng.gen();
        *pixel = Rgb([r, g, b]);
    }

    img.save("test.bmp").unwrap();
}

fn main() {
    // let image_size = Size(8, 8);
    // generate_random_image(&image_size);

    let mut file = read("test.bmp").unwrap();

    let signature: &String = &file[0..2].iter().map(|&e| e as char).collect();
    let file_size: &u32 = &file[2..6].iter().map(|&e| e.to_le() as u32).sum();
    let reserved1: &u32 = &file[6..8].iter().map(|&e| e.to_le() as u32).sum();
    let reserved2: &u32 = &file[8..10].iter().map(|&e| e.to_le() as u32).sum();
    let pixel_data_offset: &u32 = &file[10..14].iter().map(|&e| e.to_le() as u32).sum();
    let dib_header_size: &u32 = &file[14..18].iter().map(|&e| e.to_le() as u32).sum();
    let width: &i32 = &file[18..22].iter().map(|&e| e.to_le() as i32).sum();
    let height: &i32 = &file[22..26].iter().map(|&e| e.to_le() as i32).sum();
    let number_of_color_planes: &u32 = &file[26..28].iter().map(|&e| e.to_le() as u32).sum();
    let bits_per_pixel: &u32 = &file[28..30].iter().map(|&e| e.to_le() as u32).sum();
    let compression_method: &u32 = &file[30..34].iter().map(|&e| e.to_le() as u32).sum();
    let image_size: &u32 = &file[34..38].iter().map(|&e| e.to_le() as u32).sum();
    let horizontal_resolution: &i32 = &file[38..42].iter().map(|&e| e.to_le() as i32).sum();
    let vertical_resolution: &i32 = &file[42..46].iter().map(|&e| e.to_le() as i32).sum();
    let colors_in_palette: &u32 = &file[46..50].iter().map(|&e| e.to_le() as u32).sum();
    let important_colors: &u32 = &file[50..54].iter().map(|&e| e.to_le() as u32).sum();

    println!("Signature: {:?}", signature);
    println!("File size: {:?}", file_size);
    println!("Reserved bytes value: {:?}, {:?}", reserved1, reserved2);
    println!("Pixel data offset: {:?}", pixel_data_offset);
    println!("DIB header size: {:?}", dib_header_size);
    println!("Resolution: {:?}x{:?} [px]", width, height);
    println!("Number of color planes: {:?}", number_of_color_planes);
    println!("Bits per pixel: {:?}", bits_per_pixel);
    println!("Compression method: {:?}", compression_method);
    println!("Image size: {:?} [bytes]", image_size);
    println!(
        "Resolution: {:?}x{:?} [pixel per metre]",
        horizontal_resolution, vertical_resolution
    );
    println!("Colors in palette: {:?}", colors_in_palette);
    println!("Important colors: {:?}", important_colors);
}
