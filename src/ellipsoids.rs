use crate::vector::vec;
use crate::rays::ray;
use crate::primitives::{prim, intersection};
use std::f64::*;

pub struct ellipsoid{
    pub center: vec,
    pub stretch: vec,
}

impl ellipsoid{
    pub fn new(center: vec, stretch: vec) -> ellipsoid{
        ellipsoid{center, stretch}
    }

    pub fn normal(&self, point: &vec) -> vec {
        return vec::new((vec::subtr(*point, self.center)).x / self.stretch.x, 
        (vec::subtr(*point, self.center)).y / self.stretch.y, 
        (vec::subtr(*point, self.center)).z / self.stretch.z).unit();
    }
}

impl intersection for ellipsoid{
    fn getIntersection(&self, ray: &ray, start: f64, end: f64) -> (bool, prim) {
        
        let discrim = (2.0 * (vec::subtr(ray.o(), self.center)).dot(ray.d()) 
        * 2.0 * (vec::subtr(ray.o(), self.center)).dot(ray.d())) 
        - ((4.0) * (ray.d().dot(ray.d())) *
         (vec::subtr(ray.o(), self.center)).dot((vec::subtr(ray.o(), self.center))) 
         - self.stretch.dot(self.stretch));

        if discrim > 0.0 {
            let t1 = ((-2.0 * (vec::subtr(ray.o(), self.center)).dot(ray.d())) - discrim.sqrt()) / (2.0 * ray.d().dot(ray.d()));
            let t2 = (-(2.0 * (vec::subtr(ray.o(), self.center)).dot(ray.d())) + discrim.sqrt()) / (2.0 * ray.d().dot(ray.d()));
            let t = if t1 < end && t1 > start { t1 } else { t2 };
            if t < end && t > start {
                return (true, prim::new(t,ray.depth(t),vec::new(vec::subtr(ray.depth(t), self.center).x()/ self.stretch.x(),
                     vec::subtr(ray.depth(t), self.center).y()/ self.stretch.y(), 
                    vec::subtr(ray.depth(t), self.center).z()/ self.stretch.z())
                ));
            }
        }
        return (false, prim::new(end, vec::new(0.0, 0.0, 0.0), vec::new(0.0, 0.0, 0.0)),
        );
    }
}
