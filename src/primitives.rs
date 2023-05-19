use crate::rays::ray;
use crate::vector::vec;
use crate::ellipsoids::ellipsoid;
pub struct prim{
    pub t: f64,
    pub p: vec,
    pub norm: vec,
}

impl prim{
    fn new(t: f64, p: vec, norm: vec) -> prim{
        return prim{t, p, norm};
    }
}

pub trait intersection{
    fn getIntersection(ray: ray, t_min: f64, t_max: f64) -> bool;
}




