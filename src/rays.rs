use crate::vector::vec;

#[derive(Copy, Clone)]

pub struct ray{
    pub o: vec, pub d: vec 
}

impl ray {
    pub fn new(pt: vec, dir: vec) -> ray {
        return ray { o: pt, d: dir }
    }
    pub fn o(self) -> vec {
        return self.o.clone()
    }
    pub fn d(self) -> vec {
        return self.d.clone()
    }
    pub fn depth(self, t: f64) -> vec {
        let temp: vec = vec::mult(self.d, t);
        let result:vec = vec::add(temp, self.o.clone());
        let value: vec = vec::new(result.x(), result.y(), result.z());
        return value;
    }
}


