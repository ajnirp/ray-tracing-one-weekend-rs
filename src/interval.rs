pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self {
            min: min,
            max: max,
        }
    }

    pub fn min(&self) -> f64 { self.min }
    pub fn max(&self) -> f64 { self.max }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min { self.min }
        else if x > self.max { self.max }
        else { x }
    }
}
