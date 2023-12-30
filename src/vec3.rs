
use std::ops;

#[derive(Debug)]
pub struct Vec3 {
    e: [f32;3]
}


//Constructor
impl Vec3 {
    pub fn new(e0:f32, e1:f32, e2:f32)->Vec3 {
        Vec3{
            e:[e0, e1, e2]
        }
    }
}

//default fn add 
impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2]]
        }
    }
}
