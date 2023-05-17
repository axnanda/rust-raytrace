pub struct point{
    pub x: f64, pub y: f64, pub z: f64
}
impl point{
    //operators needed: declaration, x,y,z, addition with vector, subtraction with vector, distance to other point
    //instead of making another class for normals just compute it everytime
    fn new(x0: f64, y0: f64, z0: f64) -> point{
        point{x: x0, y: y0, z: z0}
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
    fn add(&self, other: vec) -> point{
        point{x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
    fn subtr(&self, other: vec) -> point{
        point{x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
    fn distance(&self, other: point) -> f64{
        ((self.x - other.x)*(self.x - other.x) + (self.y - other.y)*(self.y - other.y) + (self.z - other.z)*(self.z - other.z)).sqrt()
    }
}
