use std::iter::Sum;

pub mod camera;
pub mod hit;
pub mod ray;
pub mod scene;
pub mod sphere;
pub mod viewport;

pub type Point3 = nalgebra::Point3<f64>;
pub type Vector3 = nalgebra::Vector3<f64>;

pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }

    pub fn red(&self) -> u8 {
        (256.0 * self.r.clamp(0.0, 0.999)) as u8
    }

    pub fn green(&self) -> u8 {
        (256.0 * self.g.clamp(0.0, 0.999)) as u8
    }

    pub fn blue(&self) -> u8 {
        (256.0 * self.b.clamp(0.0, 0.999)) as u8
    }

    pub fn bytes(&self) -> [u8; 3] {
        [self.red(), self.green(), self.blue()]
    }

    pub fn lerp(&self, other: &Color, t: f64) -> Color {
        Color::new(
            (1.0 - t) * self.r + t * other.r,
            (1.0 - t) * self.g + t * other.g,
            (1.0 - t) * self.b + t * other.b,
        )
    }
}

impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}

impl std::ops::Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Color::new(self.r - other.r, self.g - other.g, self.b - other.b)
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, scalar: f64) -> Color {
        Color::new(self.r * scalar, self.g * scalar, self.b * scalar)
    }
}

impl std::ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, color: Color) -> Color {
        color * self
    }
}

impl std::ops::Div<f64> for Color {
    type Output = Color;

    fn div(self, scalar: f64) -> Color {
        Color::new(self.r / scalar, self.g / scalar, self.b / scalar)
    }
}

impl std::ops::Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}

impl From<Vector3> for Color {
    fn from(vector: Vector3) -> Color {
        Color::new(vector.x, vector.y, vector.z)
    }
}

impl Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Color::new(0.0, 0.0, 0.0), |acc, color| acc + color)
    }
}

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
