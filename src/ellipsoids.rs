use crate::vector::vec;
use crate::rays::ray;
use crate::primitives::{prim, intersection};

pub struct ellipsoid{
    pub center: vec,
    pub radius: f64,
}

impl ellipsoid{
    fn new(center: vec, radius: f64) -> ellipsoid{
        ellipsoid{center, radius}
    }
}

impl intersection for ellipsoid{
    fn getIntersection(c: vec, radius: f64, r: ray) -> bool {
        return (2.0 * vec::subtr(r.o(), c).dot(r.d()) *
         2.0 *  vec::subtr(r.o(), c).dot(r.d())) - 
         (4.0 * r.d().dot(r.d()) * vec::dot(vec::subtr(r.o(), c),
         vec::subtr(r.o(), c).clone()) - (radius * radius))
         > 0.0;
    }
    

}

