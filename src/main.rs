use rand::distributions::{Distribution, Uniform};

fn main() {
    println!("Hello, world!");
}

fn generate_random_number() -> f64 {
    let mut rng = rand::thread_rng();
    let seeder = Uniform::from(0.0..0.999999);
    return seeder.sample(&mut rng);
}

#[cfg(test)]
mod tests {
    use crate::{generate_random_number};

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
}