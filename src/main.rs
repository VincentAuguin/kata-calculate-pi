use rand::distributions::{Distribution, Uniform};

use point::Point2;

mod point;

const LOWER_INCLUSIVE_BOUND: f64 = 0.0;
const UPPER_INCLUSIVE_BOUND: f64 = 0.999999;

fn main() {
    println!("Hello, world!");
}

fn generate_random_number() -> f64 {
    let mut rng = rand::thread_rng();
    let seeder = Uniform::from(LOWER_INCLUSIVE_BOUND..UPPER_INCLUSIVE_BOUND);
    return seeder.sample(&mut rng);
}

fn generate_point() -> Point2 {
    return Point2 { x: generate_random_number(), y: generate_random_number() };
}

#[cfg(test)]
mod tests {
    use crate::{generate_point, generate_random_number};

    #[test]
    fn it_returns_number_between_0_and_1() {
        let n = generate_random_number();
        assert!(n >= 0.0 && n <= 1.0);
    }

    #[test]
    fn it_returns_random_numbers() {
        let n1 = generate_random_number();
        let n2 = generate_random_number();
        let n3 = generate_random_number();
        assert_ne!(n1, n2);
        assert_ne!(n1, n3);
        assert_ne!(n2, n3);
    }

    #[test]
    fn it_returns_2d_point() {
        let point = generate_point();
        assert!(point.x >= 0.0 && point.x <= 1.0);
        assert!(point.y >= 0.0 && point.y <= 1.0);
    }
}