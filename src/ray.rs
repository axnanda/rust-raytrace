pub struct ray{
    //ignore bounding boxes for now, instead come back if preformance is too slow
    //come back to ray differentials for anti-aliasing
    pub o: point, pub d: vec
}

impl ray {
    //operators needed: declaration, origin, direction, depth
    pub fn new(pt: point, dir: vec) -> ray {
        ray { o: pt, d: dir }
    }
    pub fn o(&self) -> point {
        self.o
    }
    pub fn d(&self) -> vec {
        self.d
    }
    pub fn depth(&self, t: f64) -> point {
        self.o + (self.d * t)
    }
}


