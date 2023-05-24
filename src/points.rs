use crate::vector::vec;
#[derive(Clone)]
pub struct point{
    pub x: f64, pub y: f64, pub z: f64
}
impl point{

    pub fn new(x0: f64, y0: f64, z0: f64) -> point{
        return point{x: x0, y: y0, z: z0}
    }
    pub fn x(&self) -> f64{
        return self.x
    }
    pub fn y(&self) -> f64{
        return self.y
    }
    pub fn z(&self) -> f64{
        return self.z
    }
    pub fn add(&self, other: vec) -> point{
        return point{x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
    pub fn subtr(&self, other: vec) -> point{
        return point{x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
    pub fn subtrpoints(&self, other: point) -> point{
        return point{x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
    pub fn distance(&self, other: point) -> f64{
        return ((self.x - other.x)*(self.x - other.x) + (self.y - other.y)*(self.y - other.y) + (self.z - other.z)*(self.z - other.z)).sqrt()
    }
}
