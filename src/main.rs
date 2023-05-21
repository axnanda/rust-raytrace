mod vector;
mod rays;
mod points;
mod primitives;
mod ellipsoids;
mod projectivecamera;


use image::{RgbImage, ImageBuffer, Rgb};

use crate::vector::vec;
use crate::primitives::prim;
use crate::points::point;
use crate::rays::ray;
use crate::ellipsoids::ellipsoid;
use crate::primitives::intersection;
use crate::projectivecamera::projcam;
use rand::prelude::*;

//things to come back to: 
//bounding boxes, transformations, 
fn color(r: &ray, objects: &[ellipsoid]) -> vec {
    for object in objects{
        let (hit, prim) = (object).getIntersection(&r, 0.001, 100000.0);
        if hit{
            let ir = 0.0 as f64;
            let ig = 255 as f64;
            let ib = 0.0 as f64;
            return vec::new(ir, ig, ib);
        }
    }
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
    let objects: Vec<ellipsoid> = vec![
        ellipsoid::new(vec::new(0.0, 0.0, -1.0), 0.5),
    ];
    let projcam = projcam::new();

    for (x, y, pixel) in buffer.enumerate_pixels_mut(){
        let mut coloratpixel = vec::new(0.0, 0.0, 0.0);
        for _ in 1..=100 {
            let randx: f64 = rand::thread_rng().gen_range(0.0..=1.0);
            let randy: f64 = rand::thread_rng().gen_range(0.0..=1.0);
            let ra: vec = vec::add(startingPoint, vec::mult(horizontal, ((x as f64 + randx) / width as f64)));
            let r: ray = ray::new(rayOrigin, vec::add(ra, vec::mult(vertical, ((y as f64 + randy) / height as f64))));
            coloratpixel = vec::add(coloratpixel, color(&r, &objects));
        }
        let average = vec::div(coloratpixel, 100.0);
        let red = (255 as f64 * average.x) as u8;
        let green = (255 as f64 * average.y) as u8;
        let blue = (255 as f64 * average.z) as u8;
        *pixel = Rgb([red, green, blue]);
    }

    //i think the problem is somewhere in vector 
    buffer.save("image4.png").unwrap();
    
    //pbr cleanup
    //return result with error loggging
    //start with converting ppm image to jpg
    //multithreading so that it preforms better
   
}
