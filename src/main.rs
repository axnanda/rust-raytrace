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

fn color(r: &ray, ell: &[ellipsoid], rects: &[rectangle], solid: &[ellipsoid], lights: &[ellipsoid], numRecursive: i32) -> vec {
    let mut rng = rand::thread_rng();
    if(numRecursive <= 0){
        return vec::new(1.0, 1.0, 1.0);
    }
   
   

    let mut endcolor = vec::new(0.0, 0.0, 0.0);

    for object in lights{
        let (hit, prim) = (object).getIntersection(&r, 0.001, 1000.0);
        if hit{
            endcolor = vec::add(endcolor, vec::new(5.0, 5.0, 5.0));
        }
    }

    for object in ell{
        let (hit, prim) = (object).getIntersection(&r, 0.001, 10000000.0);
        if hit{
            let mut scattered = ray::new(vec::new(0.0, 0.0, 0.0), vec::new(0.0, 0.0, 0.0));
            let mut attenuation = vec::new(0.0, 0.0, 0.0);
            if scatter(r, object, &mut attenuation, &mut scattered) {
                return color(&scattered, ell, rects, solid, lights, numRecursive - 1);
            }

        } 
    }

    for object in rects{
        let (hit, prim) = (object).getIntersection(&r, 0.001, 1000.0);
        if hit{
            let ir = 0.0 as f64;
            let ig = 255 as f64;
            let ib = 0.0 as f64;
            return vec::new(ir, ig, ib);
        }
    }

    for object in solid{
        let mut r1: f64 = rng.gen_range(-1.0..=1.0);
        let mut r2: f64 = rng.gen_range(-1.0..=1.0);
        let mut r3: f64 = rng.gen_range(-1.0..=1.0);
        let mut random: vec = vec::new(r1, r2, r3);
        while(random.length()*random.length()>1.0){
            r1 = rng.gen_range(-1.0..=1.0);
            r2 = rng.gen_range(-1.0..=1.0);
            r3 = rng.gen_range(-1.0..=1.0);
            random = vec::new(r1, r2, r3);
        }

        let mut random = vec::new(r1, r2, r3);
        let (hit, prim) = (object).getIntersection(&r, 0.001, 10000000.0);
        if hit{
            let lambertanian: vec = vec::add(vec::add( prim.p, ellipsoid::normal(object, &prim.p)), 
            random);
            let pt: point = point::new(lambertanian.x(), lambertanian.y(), lambertanian.z());
            let depth = vec::new(r.o().x() - pt.x(), r.o().y() - pt.y(), r.o().z() - pt.z());
            let newRay = ray::new(r.o(), depth);
            endcolor = vec::mult(color(&newRay, ell, rects, solid, lights, numRecursive - 1), 0.5);
            //return (vec::mult(color(&newRay, ell, rects, solid, lights, numRecursive-1),0.5));
        }
    }
    
    return endcolor;
  
}


fn refract(uv: &vec, n: &vec, e: f64) -> vec {
    return vec::add((vec::mult((vec::add(vec::mult(*n, (f64::min(-uv.dot(*n), 1.0))),
    *uv)), e)), (vec::mult(*n,-f64::sqrt(f64::abs(1.0 -
        ((vec::mult((vec::add(vec::mult(*n, (f64::min(-uv.dot(*n), 1.0))),
   *uv)), e)).length() * 
   (vec::mult((vec::add(vec::mult(*n, (f64::min(-uv.dot(*n), 1.0))),
   *uv)), e)).length()))))));
}

fn reflect(v: &vec, n: &vec) -> vec {
    return vec::subtr(*v, vec::mult(*n, 2.0 * v.dot(*n)));
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

fn scatter(r: &ray, rec: &ellipsoid, attenuation: &mut vec, scat: &mut ray) -> bool {
    *attenuation = vec::new(1.0, 1.0, 1.0);
    let ratioofrefraction = if rec.frontface(&r.o()) { 1.0 / 1.5 } else { 1.5 };

    let refracts = ratioofrefraction *  (f64::sqrt(1.0 - (f64::min(-(r.d().unit()).dot(rec.normal(&r.o())), 1.0)) 
    * (f64::min(-(r.d().unit()).dot(rec.normal(&r.o())), 1.0)))) > 1.0;
    let direction: vec;
    let mut rng = rand::thread_rng();
    if refracts || reflectance((f64::min(-(r.d().unit()).dot(rec.normal(&r.o())), 1.0)), ratioofrefraction) > rng.gen_range(0.0..=1.0){
        direction = reflect(&(r.d().unit()), &rec.normal(&r.o()));
    } else {
        direction = refract(&(r.d().unit()), &rec.normal(&r.o()), ratioofrefraction);
    }
    *scat = ray::new(r.o(), direction);
    return true;
}

fn main(){
    const width: u32 = 250;
    const height: u32 = 175;
    let mut buffer: RgbImage = ImageBuffer::new(width, height);
    let ellipsoids: Vec<ellipsoid> = vec![
        ellipsoid::new(vec::new(0.0, 0.0, -1.0), vec::new(0.5, 0.5, 0.5)),
    ];
    let rectangles: Vec<rectangle> = vec![
    ];

    let solids: Vec<ellipsoid> = vec![
        ellipsoid::new(vec::new(0.0, -100.5, -1.0), vec::new(100.0,100.0,100.0)),
        ellipsoid::new(vec::new(-1.0, 0.0, -1.0), vec::new(0.5, 0.5, 0.5)),
       // ellipsoid::new(vec::new(1.0, 0.0, -1.0), vec::new(0.5, 1.0, 0.5)),
    ];

    let light: Vec<ellipsoid> = vec![
        ellipsoid::new(vec::new(1.0, 0.0, -1.0), vec::new(0.5, 1.0, 0.5)),
    ];

    let camera = projcam::new(vec::new(0.0, 0.0, 0.0), 
    vec::new(-2.0, -1.0, -1.0), vec::new(4.0, 0.0, 0.0),
     vec::new(0.0, 2.0, 0.0), vec::new(1.0, 0.0, 0.0), 
     vec::new(0.0, 1.0, 0.0), vec::new(0.0, 0.0, 1.0),  0.0); 

    for (x, y, pixel) in buffer.enumerate_pixels_mut(){  
        let mut coloratpixel = vec::new(0.0, 0.0, 0.0);
        for _ in 1..=100 {    
            let randx: f64 = rand::thread_rng().gen_range(0.0..=1.0);
            let randy: f64 = rand::thread_rng().gen_range(0.0..=1.0);
            let u = (x as f64 + randx) / width as f64;
            let v = (y as f64 + randy) / height as f64;
            let r = camera.generateray(u, v);
            coloratpixel = vec::add(coloratpixel, color(&r, &ellipsoids, &rectangles, &solids, &light, 50));
        }
       
        let average = vec::div(coloratpixel, 100.0);
        let red = (255 as f64 * average.x) as u8;
        let green = (255 as f64 * average.y) as u8;
        let blue = (255 as f64 * average.z) as u8;

        *pixel = Rgb([red, green, blue]);
        
    }
    buffer.save("image18.png").unwrap();
    
}
