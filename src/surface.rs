use crate::rays::ray;
use crate::vectors::vec;

pub struct surface{
    pub ray: ray,
}

impl surface{
    fn new(t: f64, p: vec, normal: vec) -> intersection {
        intersection { t, p, normal }
    }
}

pub trait intersection{
    fn hit(ray: ray, t_min: f64, t_max: f64) -> bool;
}
