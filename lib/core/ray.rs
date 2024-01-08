
use super::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn ray(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(self) -> Vec3 {
        self.origin
    }

    pub fn direction(self) -> Vec3 {
        self.direction
    }

    pub fn point_at_parameter(self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_ray_origin() {}

    #[test]
    fn test_ray_direction() {}

    #[test]
    fn test_ray_point_at_parameter() {}
}
