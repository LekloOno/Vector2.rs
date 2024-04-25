pub mod quick_hand {
    use crate::vector2::Vector2;
    use std::f32::consts::PI;

    impl Vector2 {
        pub fn wide_angle(&self, other: &Self) -> f32 {
            let mg_base = self.magnitude() * other.magnitude();
            if mg_base == 0. { return 0.; }
    
            (self.x()*other.y() - self.y()*other.x()).atan2(self.x()*other.x() + self.y()*other.y()).rem_euclid(2.*PI)
        }
    
        pub fn distance(&self, other: &Self) -> f32 {
            (Vector2::new(self.x()-other.x(), self.y() - other.y())).magnitude()
        }
    
        pub fn direction(&self, other: &Self) -> Self {
            (Vector2::new(other.x()-self.x(), other.y() - self.y())).normalized()
        }
    
        pub fn to_angle(&self) -> f32 {
            self.y().atan2(self.x())
        }
    }
}

#[cfg(test)]
mod tests {
    //  ___________________________
    //
    //  TESTS OVER QUICK HAND TOOLS
    //  ___________________________
    //  Vector2 Implements :
    //  - distance      (Self, Self) -> f32
    //  - direction     (Self, Self) -> Self
    //  - wide_angle    (Self, Self) -> f32
    //  - to_angle      (Self) -> f32
    
    use crate::vector2::Vector2;
    use std::f32::consts::PI;
    use crate::test_tools;

    fn test_distance_procedure(vec1: Vector2, vec2: Vector2, expected_distance: f32) {
        let distance_from_vec1 = vec1.distance(&vec2);
        let distance_from_vec2 = vec2.distance(&vec1);
        assert_eq!(distance_from_vec1, distance_from_vec2);
        test_tools::assert_approx_eq!(expected_distance, &distance_from_vec1);
    }

    #[test]
    fn vector2_should_implement_distance() {
        test_distance_procedure(Vector2::new(0., 0.), Vector2::new(1., 0.), 1.);
        test_distance_procedure(Vector2::new(2., -4.), Vector2::new(-1., 0.), 5.);
    }
    
    fn test_direction_procedure(vec1: Vector2, vec2: Vector2, expected_direction: Vector2) {
        assert_eq!(expected_direction, vec1.direction(&vec2));
        let neg_direction = Vector2::new(-expected_direction.x(), -expected_direction.y());
        assert_eq!(neg_direction, vec2.direction(&vec1));
    }

    #[test]
    fn vector2_should_implement_direction() {
        test_direction_procedure(
            Vector2::new(10., 15.),
            Vector2::new(10., 25.),
            Vector2::new(0., 1.)
        );

        
        let expected_component = 1./2_f32.sqrt();
        test_direction_procedure(
            Vector2::new(1., 0.),
            Vector2::new(3., 2.),
            Vector2::new(expected_component, expected_component)
        );
    }

    #[test]
    fn vector2_should_implement_wide_angle() {
        let vec1 = Vector2::new(1., 0.);
        let vec2 = Vector2::new(0., 1.);
        assert_eq!(PI/2., vec1.wide_angle(&vec2));

        let vec2 = Vector2::new(0., -1.);
        assert_eq!(3.*PI/2., vec1.wide_angle(&vec2));

        let vec2 = Vector2::new(-1., 0.);
        assert_eq!(PI, vec1.wide_angle(&vec2));
    }

    #[test]
    fn vector2_should_implement_to_angle() {
        let vec1 = Vector2::new(1., 0.);
        assert_eq!(0., vec1.to_angle());

        let vec1 = Vector2::new(0., 1.);
        assert_eq!(PI/2., vec1.to_angle());

        let vec1 = Vector2::new(0., -1.);
        assert_eq!(-PI/2., vec1.to_angle());

        let vec1 = Vector2::new(-1., 1.);
        assert_eq!(3.*PI/4., vec1.to_angle());
    }
}