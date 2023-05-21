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
    pub fn new() -> projcam{
        projcam{o: vec::new(0.0, 0.0, 0.0), lowerleft: vec::new(-2.0, -1.0, -1.0), horizontal: vec::new(4.0, 0.0, 0.0), vertical: vec::new(0.0, 2.0, 0.0), u: vec::new(1.0, 0.0, 0.0), v: vec::new(0.0, 1.0, 0.0), w: vec::new(0.0, 0.0, 1.0), lensradius: 0.0}
    }

    pub fn generateray(&self, u: f64, v: f64) -> ray {
        return ray::new(self.o, vec::subtr(vec::add(vec::add(self.lowerleft, vec::mult(self.horizontal, u ) ), vec::mult(self.vertical, v)) , self.o));
        
    }
}