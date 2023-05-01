pub struct Rectangle {
    width: f64,
    height: f64
}

pub struct Circle {
    radius: f64
}

pub enum Feature {
    Area,
    Perimeter
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    pub fn get_feature(&self, feature: Feature) -> f64 {
        match feature {
            Feature::Area => self.calc_area(),
            Feature::Perimeter => self.calc_perimeter()
        }
    }

    fn calc_area(&self) -> f64 {
        self.width * self.height
    }

    fn calc_perimeter(&self) -> f64 {
        2.0 * self.width + 2.0 * self.height
    }
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }

    pub fn get_feature(&self, feature: Feature) -> f64 {
        match feature {
            Feature::Area => self.calc_area(),
            Feature::Perimeter => self.calc_circumference()
        }
    }

    fn calc_area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }

    fn calc_circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_rect_new() {
        let rect = Rectangle::new(1.2, 3.4);
        assert_eq!(rect.width, 1.2);
        assert_eq!(rect.height, 3.4);
    }
    
    #[test]
    fn test_rectangle_calc_area() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let width = rng.gen_range(0.0..100.0);
            let height = rng.gen_range(0.0..100.0);
            let rect = Rectangle::new(width, height);

            assert_eq!(rect.calc_area(), width * height);
        }
    }

    #[test]
    fn test_rectangle_calc_perimeter() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let width = rng.gen_range(0.0..100.0);
            let height = rng.gen_range(0.0..100.0);
            let rect = Rectangle::new(width, height);

            assert_eq!(rect.calc_perimeter(), 2.0 * (width + height));
        }
    }

    #[test]
    fn test_rectangle_get_feature() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let width = rng.gen_range(0.0..100.0);
            let height = rng.gen_range(0.0..100.0);
            let rect = Rectangle::new(width, height);

            assert_eq!(rect.get_feature(Feature::Area), rect.calc_area());
            assert_eq!(rect.get_feature(Feature::Perimeter), rect.calc_perimeter());
        }
    }

    #[test]
    fn test_circle_new() {
        let circle = Circle::new(3.14);
        assert_eq!(circle.radius, 3.14);
    }

    #[test]
    fn test_circle_calc_area() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let radius = rng.gen_range(0.0..100.0);
            let circle = Circle::new(radius);

            assert_eq!(circle.calc_area(), std::f64::consts::PI * radius.powi(2));
        }
    }

    #[test]
    fn test_circle_calc_circumference() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let radius = rng.gen_range(0.0..100.0);
            let circle = Circle::new(radius);

            assert_eq!(circle.calc_circumference(), 2.0 * std::f64::consts::PI * radius);
        }
    }

    #[test]
    fn test_circle_get_feature() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let radius = rng.gen_range(0.0..100.0);
            let circle = Circle::new(radius);
            
            assert_eq!(circle.get_feature(Feature::Area), circle.calc_area());
            assert_eq!(circle.get_feature(Feature::Perimeter), circle.calc_circumference());
        }
    }
}
