pub struct vec{
    pub x: f64, pub y: f64, pub z: f64
}
impl vec{

//operators needed: +, -, /, *, dot prod, cross prod, absolute dot prod, normalization (unit vector), length
//points?
    fn new(x0: f64, y0: f64, z0: f64) -> vec{
        vec{x: x0, y: y0, z: z0}
    }
    fn x(&self) -> f64{
        self.x
    }
    fn y(&self) -> f64{
        self.y
    }
    fn z(&self) -> f64{
        self.z
    }
    fn add(&self, other: vec) -> vec{
        vec{x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
    fn subtr(&self, other: vec) -> vec{
        vec{x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
    fn div(&self, denom: f64) -> vec{
        vec{x: self.x / denom, y: self.y / denom, z: self.z / denom}
    }
    fn mult(&self, coeff: f64) -> vec{
        vec{x: self.x * coeff, y: self.y * coeff, z: self.z * coeff}
    }
    fn length(&self) -> f64{
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }
    fn dot(&self, other: vec) -> f64{
        self.x*other.x + self.y*other.y + self.z*other.z
    }
    fn cross(&self, other: vec) -> vec{
        vec{x: self.y*other.z - self.z*other.y, y: self.z*other.x - self.x*other.z, z: self.x*other.y - self.y*other.x}
    }
    fn absdot(&self, other: vec) -> f64{
        (self.x*other.x + self.y*other.y + self.z*other.z).abs()
    }
    fn crossmag(&self, other: vec) -> f64{
        (self.y*other.z - self.z*other.y).abs() + (self.z*other.x - self.x*other.z).abs() + (self.x*other.y - self.y*other.x).abs()
    }
    fn unit(&self) -> vec{
        self.div(self.length())
    }



}
