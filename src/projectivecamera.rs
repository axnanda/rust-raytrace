use crate::rays::ray;
use crate::vector::vec;

//needed actions: generate ray, generate raster, generate camera samples
//perspective camera
//depth of field

pub struct projcam {
    pub o: vec,
    pub lowerleft: vec,
    pub horizontal: vec,
    pub vertical: vec,
    pub u: vec,
    pub v: vec,
    pub w: vec,
    pub lensradius: f64,
}

impl projcam{
    pub fn new(o:vec, lowerleft: vec, horizontal: vec, vertical: vec, u: vec, v: vec, w: vec, lensradius: f64) -> projcam{
        return projcam{o: o, lowerleft: lowerleft, horizontal: horizontal, vertical: vertical, u: u, v: v, w: w, lensradius: lensradius}
    }

    pub fn generateray(&self, u: f64, v: f64) -> ray {
        return ray::new(self.o, vec::subtr(vec::add(vec::add(self.lowerleft, vec::mult(self.horizontal, u ) ), vec::mult(self.vertical, v)) , self.o));
        
    }
}