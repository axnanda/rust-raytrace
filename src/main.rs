mod vector;
mod rays;
mod points;
mod primitives;
mod ellipsoids;

use image::{RgbImage, ImageBuffer, Rgb};

use crate::vector::vec;
use crate::primitives::prim;
use crate::points::point;
use crate::rays::ray;

//things to come back to: 
//bounding boxes, transformations, 
fn color(r: ray) -> vec {

    //come back to fix this
    /*
    if(ellipsoids(vec::new(0.0,0.0,-1.0), 0.5, r)){
        let ir = 0.0 as f64;
        let ig = 0.0 as f64;
        let ib = 255 as f64;
        return vec::new(ir, ig, ib);
    }
    */
    return vec::new(1.0, 1.0, 1.0);
}

fn main(){
    const width: u32 = 1000;
    const height: u32 = 500;
    let mut buffer: RgbImage = ImageBuffer::new(width, height);
    let startingPoint: vec = vec::new(-2.0, -1.0, -1.0);
    let rayOrigin = vec::new(0.0, 0.0, 0.0);
    let horizontal = vec::new(4.0, 0.0, 0.0);
    let vertical = vec::new(0.0, 2.0, 0.0);

    for (x, y, pixel) in buffer.enumerate_pixels_mut(){
        let ra:vec = vec::add(startingPoint, vec::mult(horizontal, (x as f64 / width as f64)));
        let r:ray = ray::new(rayOrigin, vec::add(ra, vec::mult(vertical, (y as f64 / height as f64))));  
        let red = (255 as f64 * color(r).x) as u8;
        let green = (255 as f64 * color(r).y) as u8;
        let blue = (255 as f64 * color(r).z) as u8;
        *pixel = Rgb([red, green, blue]);
    }

    //i think the problem is somewhere in vector 
    buffer.save("image3.png").unwrap();
    
    //pbr cleanup
    //return result with error loggging
    //start with converting ppm image to jpg
    //multithreading so that it preforms better
   
}
