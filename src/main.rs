use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use image::{RgbImage, ImageBuffer, Rgb};

fn main(){
    const width: u32 = 500;
    const height: u32 = 500;
    
    //let mut buffer = String::from(format!("P3\n{} {}\n255\n", width, height));
    //let reader = BufReader::new(file);
    
    let mut buffer: RgbImage = ImageBuffer::new(width, height);
    //make buffer of strings which everything will be put into
    //tuple with for loop to iterate through each pixel
    for (x, y, pixel) in buffer.enumerate_pixels_mut(){
        let r = (255.999 * (x as f64 / (width-1) as f64)) as u8;
        let g = (255.999 * (y as f64 / (height-1) as f64)) as u8;
        let b = 0 as u8;
        *pixel = Rgb([r, g, b]);
        //IS THERE ERROR IN BUFFER???
    }
    
    buffer.save("image.png").unwrap();
    /*
    //write to file system
    std::fs::write("rawout.ppm", buffer).expect("error");
    let file = File::open("rawout.ppm").expect("error on file");

    let decoder = PNMDecoder::new(reader).expect("error on reader");
    let image = decoder.read_image().expect("error on image");
    let dynamic_image = DynamicImage::ImageRgb8(image.to_rgb8());
    let path = Path::new("output.jpg");
    let output = File::create(path).expect("error on output");
    let mut encoder = JpegEncoder::new(output);
    encoder
        .encode_image(&dynamic_image)
        .expect("Failed to encode image");

     */
    //intitalize options
    //declare vector string for filenames
    //pbr init
    //pbr cleanup
    //return result with error loggging
    //start with converting ppm image to jpg


    //multithreading so that it preforms better
   
}

