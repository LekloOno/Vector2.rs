pub mod vector2 {
    fn magnitude_of(vector: &Vector2) -> f32 {
        magnitude(vector.x, vector.y)
    }

    fn magnitude(x: f32, y: f32) -> f32 {
        (x.powf(2.) + y.powf(2.)).sqrt()
    }
    
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Vector2 {
        x: f32,
        y: f32,
        magnitude: f32,
    }

    impl Vector2 {
        pub fn new(x: f32, y: f32) -> Vector2 {
            let magnitude = magnitude(x, y);
            Vector2{x, y, magnitude}
        }

        pub fn x(&self) -> f32 {
            self.x
        }

        pub fn y(&self) -> f32 {
            self.y
        }
        
        pub fn magnitude(&self) -> f32 {
            self.magnitude
        }
        
        pub fn set_x(&mut self, x: f32) {
            self.x = x;
            self.magnitude = magnitude_of(&self);
        }
        
        pub fn set_y(&mut self, y: f32) {
            self.y = y;
            self.magnitude = magnitude_of(&self);
        }
    }
}

mod ops;
mod core;
mod quick_hand;
mod morph;

#[cfg(test)]
mod test_tools;

#[cfg(test)]
mod tests {
    use super::vector2::Vector2;

    //  _____________________
    //  
    //  BASIC BEHAVIORS TESTS
    //  _____________________
    //  Vector2 implements :
    //  - vector components datas encapsulation
    //  - vector magnitude "propertization"

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
}
