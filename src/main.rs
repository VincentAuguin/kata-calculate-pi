use rand::distributions::{Distribution, Uniform};

use point::Point2;

mod point;

const LOWER_INCLUSIVE_BOUND: f64 = 0.0;
const UPPER_INCLUSIVE_BOUND: f64 = 0.999999;

fn main() {
    println!("Hello, world!");
}

fn count_points_in_circle(points: Vec<Point2>) -> usize {
    return points
        .iter()
        .filter(|p| p.x.powi(2) + p.y.powi(2) < 1.0)
        .count();
}

fn generate_points(n: i32) -> Vec<Point2> {
    let mut points = Vec::new();
    let mut i = 0;
    while i < n {
        points.push(generate_point());
        i = i + 1;
    }
    return points;
}

fn generate_point() -> Point2 {
    return Point2 { x: generate_random_number(), y: generate_random_number() };
}

fn generate_random_number() -> f64 {
    let mut rng = rand::thread_rng();
    let seeder = Uniform::from(LOWER_INCLUSIVE_BOUND..UPPER_INCLUSIVE_BOUND);
    return seeder.sample(&mut rng);
}

#[cfg(test)]
mod tests {
    use crate::{count_points_in_circle, generate_point, generate_points, generate_random_number};
    use crate::point::Point2;

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


    #[test]
    fn it_returns_list_of_points() {
        let points = generate_points(10);
        assert_eq!(points.len(), 10);
        let points = generate_points(20);
        assert_eq!(points.len(), 20);
    }

    #[test]
    fn it_returns_number_of_point_in_circle_of_radius_1() {
        let points_in_circle = count_points_in_circle(Vec::new());
        assert_eq!(points_in_circle, 0);

        let mut points = Vec::new();
        points.push(Point2 { x: 0.0, y: 0.0 });     // INNER
        points.push(Point2 { x: 0.25, y: 0.1 });    // INNER
        points.push(Point2 { x: 0.1, y: 0.1 });     // INNER
        points.push(Point2 { x: 0.99, y: 0.99 });   // OUTER
        let points_in_circle = count_points_in_circle(points);
        assert_eq!(points_in_circle, 3);
    }
}