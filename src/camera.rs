

use crate::{vec3::Vec3, ray::Ray};

pub struct Camera{

    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,

}

impl Camera{
    pub fn camera()->Camera{
       Camera{
        lower_left_corner:  Vec3::new(-2.0, -1.0, -1.0),
        horizontal: Vec3::new(4.0, 0.0, 0.0),
        vertical:  Vec3::new(0.0, 2.0, 0.0),
        origin: Vec3::new(0.0, 0.0, 0.0),

       }

    }

    pub fn origin(self)->Vec3{
        self.origin
    }

    pub fn lower_left_corner(self)->Vec3{
        self.lower_left_corner
    }

    pub fn horizontal(self)->Vec3{
        self.horizontal
    }

    pub fn vertical(self)->Vec3{
        self.vertical
    }


    pub fn get_ray(&self, u: f32 ,v: f32)->Ray{
       Ray::ray(
        self.origin,
        self.lower_left_corner + u *self.horizontal + v * self.vertical -self.origin,
        )
    }
}


