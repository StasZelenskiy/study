use shapes::{Circle, Rectangle, Feature};
use rand::prelude::*;

#[test]
fn test_rectangle_area() {
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let width = rng.gen_range(0.0..100.0);
        let height = rng.gen_range(0.0..100.0);
        let rect = Rectangle::new(width, height);
        assert_eq!(rect.get_feature(Feature::Area), width * height);
    }
}

#[test]
fn test_rectangle_perimeter() {
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let width = rng.gen_range(0.0..100.0);
        let height = rng.gen_range(0.0..100.0);
        let rect = Rectangle::new(width, height);
        assert_eq!(rect.get_feature(Feature::Perimeter), 2.0 * (width + height));
    }
}

#[test]
fn test_circle_area() {
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let radius = rng.gen_range(0.0..100.0);
        let circle = Circle::new(radius);
        assert_eq!(circle.get_feature(Feature::Area), std::f64::consts::PI * radius.powi(2));
    }
}

#[test]
fn test_circle_circumference() {
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let radius = rng.gen_range(0.0..100.0);
        let circle = Circle::new(radius);
        assert_eq!(circle.get_feature(Feature::Perimeter), 2.0 * std::f64::consts::PI * radius);
    }
}
