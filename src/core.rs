pub mod extended_ops {
    use crate::vector2::Vector2;

    impl Vector2 {
        pub fn dot_product(&self, other: &Self) -> f32 {
            self.x() * other.x() + self.y() * other.y()
        }
        
        pub fn determinent(&self, other: &Self) -> f32 {
            self.x() * other.y() - self.y() * other.x()
        }
    }
}

pub mod geometry {
    use crate::vector2::Vector2;

    impl Vector2 {
        pub fn normalized(&self) -> Self {
            if self.magnitude() == 0. { Vector2::new(0., 0.) }
            else { Vector2::new(self.x()/self.magnitude(), self.y()/self.magnitude()) } 
        }
        
        pub fn angle(&self, other: &Self) -> f32 {
            let mg_base = self.magnitude() * other.magnitude();
            if mg_base == 0. { return 0.; }
            
            (self.dot_product(other)/mg_base).acos()
        }
        
        pub fn signed_angle(&self, other: &Self) -> f32 {
            let mg_base = self.magnitude() * other.magnitude();
            if mg_base == 0. { return 0.; }
        
            (self.x()*other.y() - self.y()*other.x()).atan2(self.x()*other.x() + self.y()*other.y())
        }
    }
}


#[cfg(test)]
mod tests {
    //  _____________________
    //  
    //  TESTS OVER CORE TOOLS
    //  _____________________
    //  Vector2 Implements :
    //  - normalized        (Self) -> Self
    //  - dot_product       (Self, Self) -> f32
    //  - determinent       (Self, Self) -> f32
    //  - angle             (Self, Self) -> f32
    //  - signed_angle      (Self, Self) -> f32
    //  - projected_on      (Self, Self) -> Self
    
    use crate::vector2::Vector2;
    use std::f32::consts::PI;
    use crate::test_tools;

    #[test]
    fn vector2_should_implement_normalized() {
        let mut vec1 = Vector2::new(950., 0.);
        vec1 = vec1.normalized();

        assert_eq!(1., vec1.x());
        assert_eq!(0., vec1.y());
        test_tools::assert_approx_eq!(1., &vec1.magnitude());

        let mut vec1 = Vector2::new(1., 1.);
        vec1 = vec1.normalized();
        let exepected_xy = 1./(2_f32).sqrt();
        assert_eq!(exepected_xy, vec1.x());
        assert_eq!(exepected_xy, vec1.y());
        test_tools::assert_approx_eq!(1., &vec1.magnitude());

        let mut vec1 = Vector2::new(26.2, 12.5);
        vec1 = vec1.normalized();
        test_tools::assert_approx_eq!(1., &vec1.magnitude());
    }

    fn test_dot_product_procedure(vec1: Vector2, vec2: Vector2, expected_prod: f32) {
        let vec1prod = vec1.dot_product(&vec2);
        let vec2prod = vec2.dot_product(&vec1);
        assert_eq!(vec1prod, vec2prod);
        assert_eq!(vec1prod, expected_prod);
    }

    #[test]
    fn vector2_should_implement_dot_product() {
        test_dot_product_procedure(
            Vector2::new(1., 0.),
            Vector2::new(0., 1.),
            0.
        );

        test_dot_product_procedure(
            Vector2::new(5., 25.),
            Vector2::new(1., 0.),
            5.
        );

        test_dot_product_procedure(
            Vector2::new(5., 25.),
            Vector2::new(-1., 0.),
            -5.
        );

        test_dot_product_procedure(
            Vector2::new(5., 25.),
            Vector2::new(1., 1.),
            30.
        );
    }

    fn test_determinent_procedure(vec1: Vector2, vec2: Vector2, expected_area: f32){
        test_tools::assert_approx_eq!(expected_area, &vec1.determinent(&vec2));
        test_tools::assert_approx_eq!(-expected_area, &vec2.determinent(&vec1));
    }

    #[test]
    fn vector2_should_implement_determinent() {
        test_determinent_procedure(Vector2::new(1., 0.), Vector2::new(0., 0.), 0.);
        test_determinent_procedure(Vector2::new(1., 0.), Vector2::new(0., 1.), 1.);
        test_determinent_procedure(Vector2::new(1., 0.5), Vector2::new(1., -1.), -1.5);
    }

    #[test]
    fn vector2_should_implement_angle() {
        let vec1 = Vector2::new(1., 0.);
        let vec2 = Vector2::new(0., 1.);
        assert_eq!(PI/2., vec1.angle(&vec2));

        let vec2 = Vector2::new(0., -1.);
        assert_eq!(PI/2., vec1.angle(&vec2));

        let vec2 = Vector2::new(-1., 0.);
        assert_eq!(PI, vec1.angle(&vec2));

        let slope_component = (1./2_f32).sqrt();
        let vec2 = Vector2::new(slope_component, slope_component);
        test_tools::assert_approx_eq!(PI/4., &vec1.angle(&vec2));
    }

    #[test]
    fn vector2_should_implement_signed_angle() {
        let vec1 = Vector2::new(1., 0.);
        let vec2 = Vector2::new(0., 1.);
        assert_eq!(PI/2., vec1.signed_angle(&vec2));

        let vec2 = Vector2::new(0., -1.);
        assert_eq!(-PI/2., vec1.signed_angle(&vec2));

        let vec2 = Vector2::new(-1., 0.);
        assert_eq!(PI, vec1.signed_angle(&vec2));
    }
}