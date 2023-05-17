use crate::vector::vec;
use crate::points::point;


pub struct ray{
    //ignore bounding boxes for now, instead come back if preformance is too slow
    //come back to ray differentials for anti-aliasing
    pub o: vec, pub d: vec 
}

impl ray {
    //operators needed: declaration, origin, direction, depth
    pub fn new(pt: vec, dir: vec) -> ray {
        return ray { o: pt, d: dir }
    }
    pub fn o(&self) -> vec {
        return self.o.clone()
    }
    pub fn d(&self) -> vec {
        return self.d.clone()
    }
    pub fn depth(&self, t: f64) -> point {
        let temp: vec = vec::mult(&self.d, t);
        let result:vec = vec::add(&temp, self.o.clone());
        let value: point = point::new(result.x(), result.y(), result.z());
        return value;
    }
}


