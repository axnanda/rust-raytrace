mod vector;
mod rays;
mod points;
mod primitives;
mod ellipsoids;
mod projectivecamera;
mod rectangles;


use image::{RgbImage, ImageBuffer, Rgb};

use rand::prelude::*;
use crate::vector::vec;
use crate::primitives::prim;
use crate::points::point;
use crate::rays::ray;
use crate::ellipsoids::ellipsoid;
use crate::primitives::intersection;
use crate::projectivecamera::projcam;
use rand::prelude::*;
use crate::rectangles::rectangle;

//things to come back to: 
//bounding boxes, transformations, 
fn color(r: &ray, ell: &[ellipsoid], rec: &[rectangle]) -> vec {
    let mut rng = rand::thread_rng();
    let mut r1: f64 = rng.gen_range(-1.0..=1.0);
    let mut r2: f64 = rng.gen_range(-1.0..=1.0);
    let mut r3: f64 = rng.gen_range(-1.0..=1.0);
    let random = vec::new(r1, r2, r3);
    while(random.length()*random.length()>1.0){
        r1 = rng.gen_range(-1.0..=1.0);
        r2 = rng.gen_range(-1.0..=1.0);
        r3 = rng.gen_range(-1.0..=1.0);
    }
    //now you have a random unit vecto

    for object in ell{
        let (hit, prim) = (object).getIntersection(&r, 0.001, 1000.0);
        
        if hit{
            let lambertanian: vec = vec::add(vec::add( prim.p, ellipsoid::normal(object, &prim.p)), random);
            let pt: point = point::new(lambertanian.x(), lambertanian.y(), lambertanian.z());
            //random is a vector
            //normal is a vector
            //rec.p + rec.normal + random_in_unit_sphere()
            let depth = vec::new(pt.x()- r.o().x(), pt.y()- r.o().y(), pt.z()- r.o().z());
            let newRay = ray::new(r.o(), depth);
            //&ray(r.o(), pt - r.o())
            return (vec::mult(color(&newRay, ell, rec),0.5));
               //point3 target = rec.p + rec.normal + random_in_unit_sphere();
           //return 0.5 * ray_color(ray(rec.p, target - rec.p), world);
            //let ir = 0.0 as f64;
            //let ig = 255 as f64;
            //let ib = 0.0 as f64;
            //return vec::new(ir, ig, ib);
        }
    }
    for object in rec{
        let (hit, prim) = (object).getIntersection(&r, 0.001, 1000.0);
        if hit{
            let ir = 0.0 as f64;
            let ig = 255 as f64;
            let ib = 0.0 as f64;
            return vec::new(ir, ig, ib);
        }
    }
    //how to make it so that both interact and that it loops through both at the same time?
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
    let ellipsoids: Vec<ellipsoid> = vec![
        ellipsoid::new(vec::new(0.0, 0.0, -1.0), 0.5),
    ];
    let rectangles: Vec<rectangle> = vec![
        rectangle::new(0.0,1.0,0.0,1.0,-1.0),
    ];
    
    let projcam = projcam::new();//COME BACK TO THIS

    for (x, y, pixel) in buffer.enumerate_pixels_mut(){
        let mut coloratpixel = vec::new(0.0, 0.0, 0.0);
        for _ in 1..=100 {
            let randx: f64 = rand::thread_rng().gen_range(0.0..=1.0);
            let randy: f64 = rand::thread_rng().gen_range(0.0..=1.0);
            let ra: vec = vec::add(startingPoint, vec::mult(horizontal, ((x as f64 + randx) / width as f64)));
            let r: ray = ray::new(rayOrigin, vec::add(ra, vec::mult(vertical, ((y as f64 + randy) / height as f64))));
            coloratpixel = vec::add(coloratpixel, color(&r, &ellipsoids, &rectangles));
        }
        let average = vec::div(coloratpixel, 100.0);
        let red = (255 as f64 * average.x) as u8;
        let green = (255 as f64 * average.y) as u8;
        let blue = (255 as f64 * average.z) as u8;
        *pixel = Rgb([red, green, blue]);
    }

    //i think the problem is somewhere in vector 
    buffer.save("image7.png").unwrap();
    
    //pbr cleanup
    //return result with error loggging
    //start with converting ppm image to jpg
    //multithreading so that it preforms better
   
}
