pub mod camera;
pub mod color;
pub mod hit;
pub mod material;
pub mod ray;
pub mod scene;
pub mod sphere;
pub mod viewport;

pub type Point3 = nalgebra::Point3<f64>;
pub type Vector3 = nalgebra::Vector3<f64>;

pub mod random {
    use std::cell::Cell;

    use crate::Vector3;

    thread_local! {
        static RNG_STATE: Cell<u32> = const { Cell::new(0) };
    }

    fn rand_pcg() -> u32 {
        let state = RNG_STATE.get();
        RNG_STATE.set(state.wrapping_mul(747796405).wrapping_add(2891336453));
        let word = (state.wrapping_shr((state.wrapping_shr(28)) + 4)) ^ state;
        let word = word.wrapping_mul(277803737);
        return (word >> 22) ^ word;
    }

    pub trait Random {
        fn random() -> Self;
    }

    pub trait RandomInRange {
        fn random_in_range(min: Self, max: Self) -> Self;
    }

    pub trait RandomVector3 {
        fn random_on_hemisphere(normal: &Vector3) -> Vector3;
    }

    impl Random for f64 {
        fn random() -> Self {
            rand_pcg() as Self / u32::MAX as Self
        }
    }

    impl RandomInRange for f64 {
        fn random_in_range(min: Self, max: Self) -> Self {
            min + (max - min) * Self::random()
        }
    }

    impl Random for Vector3 {
        fn random() -> Vector3 {
            // not rejecting points outside of the unit
            // sphere is good enough and a lot faster
            let x = f64::random_in_range(-1.0, 1.0);
            let y = f64::random_in_range(-1.0, 1.0);
            let z = f64::random_in_range(-1.0, 1.0);
            return Vector3::new(x, y, z).normalize();
        }
    }

    impl RandomVector3 for Vector3 {
        fn random_on_hemisphere(normal: &Vector3) -> Vector3 {
            let unit_vector = Vector3::random();

            if unit_vector.dot(normal) > 0.0 {
                unit_vector
            } else {
                -unit_vector
            }
        }
    }
}
