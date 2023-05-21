use crate::vector::vec;
use crate::rays::ray;
use crate::primitives::{prim, intersection};
use std::f64::*;

pub struct ellipsoid{
    pub center: vec,
    pub radius: f64,
}

impl ellipsoid{
    pub fn new(center: vec, radius: f64) -> ellipsoid{
        ellipsoid{center, radius}
    }
}

impl intersection for ellipsoid{
    fn getIntersection(&self, ray: &ray, start: f64, end: f64) -> (bool, prim)
     {
        let discrim: f64 = ((vec::subtr(ray.o(), self.center)).dot(ray.d()) *
         (vec::subtr(ray.o(), self.center)).dot(ray.d())) //b^2
        - ((ray.d().dot(ray.d())) //a
        * ((vec::subtr(ray.o(), self.center)).dot((vec::subtr(ray.o(), self.center))) 
        - self.radius * self.radius));//c

        if discrim > 0.0 {
            let mut time = (-(vec::subtr(ray.o(), self.center)).dot(ray.d()) - discrim.sqrt()) / (ray.d().dot(ray.d()));
            if time < end && time > start {
                let mut object = prim::new(time, ray.depth(time), vec::div((vec::subtr(ray.depth(time), self.center)), self.radius));
                return (true, object);
            }
            time = (-(vec::subtr(ray.o(), self.center)).dot(ray.d()) + discrim.sqrt()) / (ray.d().dot(ray.d())); //check other root
            if time < end && time > start {
                let mut object2 = prim::new(time, ray.depth(time), vec::div((vec::subtr(ray.depth(time), self.center)), self.radius));
                return (true, object2);
            }
        }
        return (false, prim::new(end, vec::new(0.0, 0.0, 0.0), vec::new(0.0, 0.0, 0.0)));
    }
}
