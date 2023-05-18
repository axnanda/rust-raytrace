
#[derive(Copy, Clone)]
pub struct vec{
    pub x: f64, pub y: f64, pub z: f64
}
impl vec{

//operators needed: +, -, /, *, dot prod, cross prod, absolute dot prod, normalization (unit vector), length
//points?
    pub fn new(x0: f64, y0: f64, z0: f64) -> vec{
        return vec{x: x0, y: y0, z: z0}
    }
    pub fn x(self) -> f64{
        return self.x
    }
    pub fn y(self) -> f64{
        return self.y
    }
    pub fn z(self) -> f64{
        return self.z
    }
    pub fn add(self, other: vec) -> vec{
        return vec{x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
    pub fn subtr(self, other: vec) -> vec{
        return vec{x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
    pub fn div(self, denom: f64) -> vec{
        return vec{x: self.x / denom, y: self.y / denom, z: self.z / denom}
    }
    pub fn mult(self, coeff: f64) -> vec{
        return vec{x: self.x * coeff, y: self.y * coeff, z: self.z * coeff}
    }
    pub fn length(self) -> f64{
        return (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }
    pub fn dot(self, other: vec) -> f64{
        return self.x*other.x + self.y*other.y + self.z*other.z
    }
    pub fn cross(self, other: vec) -> vec{
        return vec{x: self.y*other.z - self.z*other.y, y: self.z*other.x - self.x*other.z, z: self.x*other.y - self.y*other.x}
    }
    pub fn absdot(self, other: vec) -> f64{
        return (self.x*other.x + self.y*other.y + self.z*other.z).abs()
    }
    pub fn crossmag(self, other: vec) -> f64{
        return (self.y*other.z - self.z*other.y).abs() + (self.z*other.x - self.x*other.z).abs() + (self.x*other.y - self.y*other.x).abs()
    }
    pub fn unit(self) -> vec{
        return self.div(self.length())
    }



}
