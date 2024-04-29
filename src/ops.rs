pub mod self_ops {
    use crate::vector2::Vector2;
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, Rem, RemAssign};

    impl Add for Vector2 {
        type Output = Self;
    
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
    
    impl Div for Vector2 {
        type Output = Self;
    
        fn div(self, other: Self) -> Self {
            Vector2::new(
                self.x() / other.x(),
                self.y() / other.y()
            )
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

    impl Mul for Vector2 {
        type Output = Self;
        
        fn mul(self, other: Self) -> Self {
            Vector2::new(
                self.x() * other.x(),
                self.y() * other.y()
            )
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

    impl Rem for Vector2 {
        type Output = Self;

        fn rem(self, other: Self) -> Self {
            Vector2::new(
                self.x() % other.x(),
                self.y() % other.y(),
            )
        }
    }

    impl RemAssign for Vector2 {
        fn rem_assign(&mut self, other: Self) {
            *self = Vector2::new(
                self.x() % other.x(),
                self.y() % other.y(),
            );
        }
    }
}

pub mod k_ops {
    use crate::vector2::Vector2;
    use std::ops::{Add, AddAssign, Sub, SubAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign};

    impl Sub<f32> for Vector2 {
        type Output = Self;
    
        fn sub(self, k: f32) -> Self {
            Vector2::new(
                self.x() - k,
                self.y() - k
            )
        }
    }

    impl SubAssign<f32> for Vector2 {    
        fn sub_assign(&mut self, k: f32) {
            *self = Vector2::new(
                self.x() - k,
                self.y() - k
            );
        }
    }

    impl Add<f32> for Vector2 {
        type Output = Self;
    
        fn add(self, k: f32) -> Self {
            Vector2::new(
                self.x() + k,
                self.y() + k
            )
        }
    }

    impl AddAssign<f32> for Vector2 {    
        fn add_assign(&mut self, k: f32) {
            *self = Vector2::new(
                self.x() + k,
                self.y() + k
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
    
    impl DivAssign<f32> for Vector2 {
        fn div_assign(&mut self, k: f32) {
            *self = Vector2::new(
                self.x() / k,
                self.y() / k
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
    
    impl MulAssign<f32> for Vector2 {
        fn mul_assign(&mut self, k: f32) {
            *self = Vector2::new(
                self.x() * k,
                self.y() * k
            );
        }
    }

    impl Rem<f32> for Vector2 {
        type Output = Self;

        fn rem(self, k: f32) -> Self {
            Vector2::new(
                self.x() % k,
                self.y() % k
            )
        }
    }

    impl RemAssign<f32> for Vector2 {
        fn rem_assign(&mut self, k: f32) {
            *self = Vector2::new(
                self.x() % k,
                self.y() % k
            );
        }
    }
}


#[cfg(test)]
mod tests {
    //  ____________________
    //
    //  TESTS OVER OPERATORS
    //  ____________________
    //  Vector2 Implements :
    //  - Add, AddAssign    (Self, Self)
    //  - Sub, SubAssign    (Self, Self)
    //  - Div, DivAssign    (Self, Self), (Self, f32)
    //  - Mul, MulAssign    (Self, Self), (Self, f32)
    //  - PartialEq         - Derived

    use crate::vector2::Vector2;
    
    #[test]
    fn vector2_should_implement_k_add() {
        let mut vec1 = Vector2::new(1., 2.);
        vec1 = vec1 + 2.;
        assert_eq!(3., vec1.x());
        assert_eq!(4., vec1.y());
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
    fn vector2_should_implement_k_add_assign() {
        let mut vec1 = Vector2::new(1., 2.);
        vec1 += 2.;
        assert_eq!(3., vec1.x());
        assert_eq!(4., vec1.y());
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
    fn vector2_should_implement_k_sub() {
        let mut vec1 = Vector2::new(1., 2.);
        vec1 = vec1 - 2.;
        assert_eq!(-1., vec1.x());
        assert_eq!(0., vec1.y());
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
    fn vector2_should_implement_k_sub_assign() {
        let mut vec1 = Vector2::new(1., 2.);
        vec1 -= 2.;
        assert_eq!(-1., vec1.x());
        assert_eq!(0., vec1.y());
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
    fn vector2_should_implement_rem(){
        let vec1 = Vector2::new(4., 2.);
        let vec2 = Vector2::new(3., 2.);
        assert_eq!(vec1 % vec2, Vector2::new(1., 0.));

        let vec1 = Vector2::new(-1., -7.);
        assert_eq!(vec1 % vec2, Vector2::new(-1., -1.));
    }

    #[test]
    fn vector2_should_implement_rem_assign(){
        let mut vec1 = Vector2::new(4., 2.);
        vec1 %= Vector2::new(3., 2.);
        assert_eq!(vec1, Vector2::new(1., 0.));

        let mut vec1 = Vector2::new(-1., -7.);
        vec1 %= Vector2::new(3., 2.);
        assert_eq!(vec1, Vector2::new(-1., -1.));
    }

    #[test]
    fn vector2_should_implement_k_rem(){
        let vec1 = Vector2::new(4., 2.);
        assert_eq!(vec1 % 3., Vector2::new(1., 2.));

        let vec1 = Vector2::new(-1., -7.);
        assert_eq!(vec1 % 3., Vector2::new(-1., -1.));
    }

    #[test]
    fn vector2_should_implement_k_rem_assign(){
        let mut vec1 = Vector2::new(4., 2.);
        vec1 %= 3.;
        assert_eq!(vec1, Vector2::new(1., 2.));

        let mut vec1 = Vector2::new(-1., -7.);
        vec1 %= 3.;
        assert_eq!(vec1, Vector2::new(-1., -1.));
    }
}