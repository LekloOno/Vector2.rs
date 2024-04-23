pub mod vector2 {
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

    fn magnitude_of(vector: &Vector2data) -> f32 {
        magnitude(vector.x, vector.y)
    }

    fn magnitude(x: f32, y: f32) -> f32 {
        (x.powf(2.) + y.powf(2.)).sqrt()
    }

    pub struct Vector2data {
        pub x: f32,
        pub y: f32,
        pub magnitude: f32,
    }

    impl Vector2data {
        pub fn new(x: f32, y: f32) -> Vector2data {
            let magnitude = magnitude(x, y);
            let vec = Vector2data{x, y, magnitude};
            vec
        }
    }

    pub struct Vector2 {
        vector: Vector2data,
    }

    impl Vector2 {
        pub fn new(x: f32, y: f32) -> Vector2 {
            let vector = Vector2data::new(x, y);
            Vector2{vector}
        }

        pub fn x(&self) -> f32 {
            self.vector.x
        }

        pub fn y(&self) -> f32 {
            self.vector.y
        }

        pub fn magnitude(&self) -> f32 {
            self.vector.magnitude
        }

        pub fn set_x(&mut self, x: f32) {
            self.vector.x = x;
            self.vector.magnitude = magnitude_of(&self.vector);
        }

        pub fn set_y(&mut self, y: f32) {
            self.vector.y = y;
            self.vector.magnitude = magnitude_of(&self.vector);
        }
    }

    impl Add for Vector2 {
        type Output = Vector2;

        fn add(self, other: Self) -> Self {
            Vector2::new(
                self.x() + other.x(),
                self.y() + other.y()
            )
        }
    }

    impl AddAssign for Vector2 {
        fn add_assign(&mut self, other: Self) {
            *self = Vector2::new(
                self.x() + other.x(),
                self.y() + other.y()
            );
        }
    }

    impl Div<f32> for Vector2 {
        type Output = Self;

        fn div(self, k: f32) -> Self {
            Vector2::new(
                self.x() / k,
                self.y() / k
            )
        }
    }

    impl Div for Vector2 {
        type Output = Self;

        fn div(self, other: Self) -> Self {
            Vector2::new(
                self.x() / other.x(),
                self.y() / other.y()
            )
        }
    }

    impl DivAssign<f32> for Vector2 {
        fn div_assign(&mut self, k: f32) {
            *self = Vector2::new(
                self.x() / k,
                self.y() / k
            );
        }
    }

    impl DivAssign for Vector2 {
        fn div_assign(&mut self, other: Self) {
            *self = Vector2::new(
                self.x() / other.x(),
                self.y() / other.y()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::vector2::Vector2;

    #[test]
    fn vector2_should_contain_the_right_data() {
        let vec1 = Vector2::new(1., 2.);
        assert_eq!(1., vec1.x());
        assert_eq!(2., vec1.y());
        let vec1 = Vector2::new(2., 1.);
        assert_eq!(2., vec1.x());
        assert_eq!(1., vec1.y());
    }

    #[test]
    fn vector2_magnitude_should_be_right_on_init() {
        let vec1 = Vector2::new(4., 3.);
        assert_eq!(5., vec1.magnitude());
    }

    #[test]
    fn vector2_setter_should_update_xy() {
        let mut vec1 = Vector2::new(1., 2.);
        vec1.set_x(3.);
        assert_eq!(3., vec1.x());
        vec1.set_y(4.);
        assert_eq!(4., vec1.y());
    }

    #[test]
    fn vector2_magnitude_should_update_on_xy_set() {
        let mut vec1 = Vector2::new(4., 1.);
        vec1.set_y(3.);
        assert_eq!(5., vec1.magnitude());
        let mut vec1 = Vector2::new(1., 4.);
        vec1.set_x(3.);
        assert_eq!(5., vec1.magnitude());
    }

    #[test]
    fn vector2_should_implement_add() {
        let vec1 = Vector2::new(1., 2.);
        let vec2 = Vector2::new(2., 3.);
        let vec3 = vec1 + vec2;
        assert_eq!(3., vec3.x());
        assert_eq!(5., vec3.y());
    }

    #[test]
    fn vector2_should_implement_add_assign() {
        let mut vec1 = Vector2::new(1., 2.);
        let vec2 = Vector2::new(2., 3.);
        vec1 += vec2;
        assert_eq!(3., vec1.x());
        assert_eq!(5., vec1.y());
    }

    #[test]
    fn vector2_should_implement_k_div() {
        let vec1 = Vector2::new(4., 2.);
        let vec2 = vec1 / 2.;
        assert_eq!(2., vec2.x());
        assert_eq!(1., vec2.y());
    }

    #[test]
    fn vector2_should_implement_vec_div() {
        let vec1 = Vector2::new(4., 2.);
        let vec2 = Vector2::new(2., 4.);
        let vec3 = vec1 / vec2;
        assert_eq!(2., vec3.x());
        assert_eq!(0.5, vec3.y());
    }

    #[test]
    fn vector2_should_implement_k_div_assign() {
        let mut vec1 = Vector2::new(4., 2.);
        vec1 /= 2.;
        assert_eq!(2., vec1.x());
        assert_eq!(1., vec1.y());
    }

    #[test]
    fn vector2_should_implement_vec_div_assign() {
        let mut vec1 = Vector2::new(4., 2.);
        let vec2 = Vector2::new(2., 4.);
        vec1 /= vec2;
        assert_eq!(2., vec1.x());
        assert_eq!(0.5, vec1.y());
    }
}
