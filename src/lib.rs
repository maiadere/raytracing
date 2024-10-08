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
        (255.0 * self.r) as u8
    }

    pub fn green(&self) -> u8 {
        (255.0 * self.g) as u8
    }

    pub fn blue(&self) -> u8 {
        (255.0 * self.b) as u8
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
