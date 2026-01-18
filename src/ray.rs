use crate::vec3::Vec3;

pub struct Ray {
    orig: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Self {
        Self {
            orig: orig,
            dir: dir,
        }
    }
    
    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + (self.dir * t)
    }

    pub fn orig(&self) -> &Vec3 { &self.orig }
    pub fn dir(&self) -> &Vec3 { &self.dir }
}
