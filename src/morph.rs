mod morph {
    use crate::vector2::Vector2;

    impl Vector2 {
        pub fn normalize(&mut self) {
            *self = self.normalized();
        }
    }
}

#[cfg(test)]
mod tests {
    //  ______________________
    //  
    //  TESTS OVER MORPH TOOLS
    //  ______________________
    //  Vector2 Implements :
    //  - normalize     (Self)
    //  - project_on    (Self, Self)

    use crate::vector2::Vector2;
    use crate::test_tools;

    #[test]
    fn vector2_should_implement_normalize() {
        let mut vec1 = Vector2::new(950., 0.);
        vec1.normalize();

        assert_eq!(Vector2::new(1., 0.), vec1);

        let mut vec1 = Vector2::new(1., 1.);
        vec1.normalize();
        let exepected_xy = 1./(2_f32).sqrt();
        assert_eq!(Vector2::new(exepected_xy, exepected_xy), vec1);

        let mut vec1 = Vector2::new(26.2, 12.5);
        vec1.normalize();
        test_tools::assert_approx_eq!(1., &vec1.magnitude());
    }
}