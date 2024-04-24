pub mod vector2 {
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

    fn magnitude_of(vector: &Vector2data) -> f32 {
        magnitude(vector.x, vector.y)
    }

    fn magnitude(x: f32, y: f32) -> f32 {
        (x.powf(2.) + y.powf(2.)).sqrt()
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
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
    
    #[derive(Debug, Copy, Clone, PartialEq)]
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

        pub fn normalized(&self) -> Self {
            if self.magnitude() == 0. { Vector2::new(0., 0.) }
            else { Vector2::new(self.x()/self.magnitude(), self.y()/self.magnitude()) } 
        }

        pub fn distance(&self, other: &Self) -> f32 {
            (*self - *other).magnitude()
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

    impl Sub for Vector2 {
        type Output = Self;

        fn sub(self, other: Self) -> Self {
            Vector2::new(
                self.x() - other.x(),
                self.y() - other.y()
            )
        }
    }

    impl SubAssign for Vector2 {
        fn sub_assign(&mut self, other: Self) {
            *self = Vector2::new(
                self.x() - other.x(),
                self.y() - other.y()
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

    impl Mul<f32> for Vector2 {
        type Output = Self;

        fn mul(self, k: f32) -> Self {
            Vector2::new(
                self.x() * k,
                self.y() * k
            )
        }
    }

    impl Mul for Vector2 {
        type Output = Self;
        
        fn mul(self, other: Self) -> Self {
            Vector2::new(
                self.x() * other.x(),
                self.y() * other.y()
            )
        }
    }

    impl MulAssign<f32> for Vector2 {
        fn mul_assign(&mut self, k: f32) {
            *self = Vector2::new(
                self.x() * k,
                self.y() * k
            );
        }
    }

    impl MulAssign for Vector2 {
        fn mul_assign(&mut self, other: Self) {
            *self = Vector2::new(
                self.x() * other.x(),
                self.y() * other.y()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::vector2::Vector2;

    const TEST_DELTA: f32 = 0.0001;

    fn assert_approx_eq(expected: f32, actual: &f32, delta: f32) {
        let delta = delta.abs();
        let bound = expected+delta;
        assert!(
            *actual < bound,
            "Value did not match expected precision : Exceeded _max_ bound\n-->\t(Value {}   >   Bound {})",
            *actual,
            bound
        );

        let bound = expected-delta;
        assert!(
            *actual > bound,
            "Value did not match expected precision : Exceeded _min_ bound\n-->\t(Value {}   <   Bound {})",
            *actual,
            bound
        );
    }

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
        let mut vec1 = Vector2::new(1., 2.);
        let vec2 = Vector2::new(2., 3.);
        vec1 = vec1 + vec2;
        assert_eq!(3., vec1.x());
        assert_eq!(5., vec1.y());
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
    fn vector2_should_implement_sub() {
        let mut vec1 = Vector2::new(2., 4.);
        let vec2 = Vector2::new(1., 2.);
        vec1 = vec1 - vec2;
        assert_eq!(1., vec1.x());
        assert_eq!(2., vec1.y());
    }

    #[test]
    fn vector2_should_implement_sub_assign() {
        let mut vec1 = Vector2::new(2., 4.);
        let vec2 = Vector2::new(1., 2.);
        vec1 -= vec2;
        assert_eq!(1., vec1.x());
        assert_eq!(2., vec1.y());
    }

    #[test]
    fn vector2_should_implement_k_div() {
        let mut vec1 = Vector2::new(4., 2.);
        vec1 = vec1 / 2.;
        assert_eq!(2., vec1.x());
        assert_eq!(1., vec1.y());
    }

    #[test]
    fn vector2_should_implement_vec_div() {
        let mut vec1 = Vector2::new(4., 2.);
        let vec2 = Vector2::new(2., 4.);
        vec1 = vec1 / vec2;
        assert_eq!(2., vec1.x());
        assert_eq!(0.5, vec1.y());
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

    #[test]
    fn vector2_should_implement_k_mul() {
        let mut vec1 = Vector2::new(4., 2.);
        vec1 = vec1 * 3.;
        assert_eq!(12., vec1.x());
        assert_eq!(6., vec1.y());
    }

    #[test]
    fn vector2_should_implement_vec_mul() {
        let mut vec1 = Vector2::new(4., 2.);
        let vec2 = Vector2::new(2., 3.);
        vec1 = vec1 * vec2;
        assert_eq!(8., vec1.x());
        assert_eq!(6., vec1.y());
    }

    #[test]
    fn vector2_should_implement_k_mul_assign() {
        let mut vec1 = Vector2::new(4., 2.);
        vec1 *= 3.;
        assert_eq!(12., vec1.x());
        assert_eq!(6., vec1.y());
    }

    #[test]
    fn vector2_should_implement_vec_mul_assign() {
        let mut vec1 = Vector2::new(4., 2.);
        let vec2 = Vector2::new(2., 3.);
        vec1 *= vec2;
        assert_eq!(8., vec1.x());
        assert_eq!(6., vec1.y());
    }

    #[test]
    fn vector2_should_implement_partial_eq() {
        let vec1 = Vector2::new(4., 2.);
        let vec2 = Vector2::new(4., 2.);
        let vec3 = Vector2::new(2., 4.);
        assert_eq!(vec1, vec2);
        assert_ne!(vec1, vec3);
    }

    #[test]
    fn vector2_should_implement_normalized() {
        let mut vec1 = Vector2::new(950., 0.);
        vec1 = vec1.normalized();

        assert_eq!(1., vec1.x());
        assert_eq!(0., vec1.y());
        assert_approx_eq(1., &vec1.magnitude(), TEST_DELTA);

        let mut vec1 = Vector2::new(1., 1.);
        vec1 = vec1.normalized();
        let exepected_xy = 1./(2_f32).sqrt();
        assert_eq!(exepected_xy, vec1.x());
        assert_eq!(exepected_xy, vec1.y());
        assert_approx_eq(1., &vec1.magnitude(), TEST_DELTA);

        let mut vec1 = Vector2::new(26.2, 12.5);
        vec1 = vec1.normalized();
        assert_approx_eq(1., &vec1.magnitude(), TEST_DELTA);
    }

    fn test_distance_quick_hand(vec1: Vector2, vec2: Vector2, expected_distance: f32) {
        let distance_from_vec1 = vec1.distance(&vec2);
        let distance_from_vec2 = vec2.distance(&vec1);
        assert_eq!(distance_from_vec1, distance_from_vec2);
        assert_approx_eq(expected_distance, &distance_from_vec1, TEST_DELTA);
    }

    #[test]
    fn vector2_should_implement_distance() {
        test_distance_quick_hand(Vector2::new(0., 0.), Vector2::new(1., 0.), 1.);
        test_distance_quick_hand(Vector2::new(2., -4.), Vector2::new(-1., 0.), 5.);
    }
}
