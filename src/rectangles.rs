use crate::vector::vec;
use crate::rays::ray;
use crate::primitives::{prim, intersection};
use std::f64::*;

pub struct rectangle{
    pub leftX: f64,
    pub rightX: f64,
    pub bottomY: f64,
    pub topY: f64,
    pub z: f64
}

impl rectangle{

    pub fn new(leftX: f64, rightX: f64, bottomY: f64, topY: f64, z:f64)-> rectangle{
        rectangle{leftX, rightX, bottomY, topY, z}
    }
}

impl intersection for rectangle{

    fn getIntersection(&self, ray: &ray, start: f64, end: f64) -> (bool, prim)
     {
        let t = (self.z - ray.o().z()) / ray.d().z();
        if((self.z - ray.o().z()) / ray.d().z() < start || (self.z - ray.o().z()) / ray.d().z() > end){
            return (false, prim::new(end, vec::new(0.0, 0.0, 0.0), vec::new(0.0, 0.0, 0.0)));
        }
        if(ray.o().x() + t * ray.d().x() < self.leftX || ray.o().x() + t * ray.d().x() > self.rightX || ray.o().y() + t * ray.d().y() < self.bottomY || ray.o().y() + t * ray.d().y() > self.topY){
            return (false, prim::new(end, vec::new(0.0, 0.0, 0.0), vec::new(0.0, 0.0, 0.0)));
        }
        return (true, prim::new(t, ray.depth(t), vec::new(0.0, 0.0, 1.0)));
    }
}
