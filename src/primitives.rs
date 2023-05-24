use crate::rays::ray;
use crate::vector::vec;
use crate::ellipsoids::ellipsoid;

pub struct prim{
    pub t: f64,
    pub p: vec,
    pub norm: vec,
}

impl prim{
    pub fn new(t: f64, p: vec, norm: vec) -> prim{
        return prim{t, p, norm};
    }
}

pub trait intersection{
    fn getIntersection(&self, ray: &ray, start: f64, end: f64) -> (bool, prim);
}




