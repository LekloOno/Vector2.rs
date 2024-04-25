pub const TEST_DELTA: f32 = 0.0001;

pub fn assert_approx_equals(expected: f32, actual: &f32, delta: f32) {
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

macro_rules! assert_approx_eq {
    ($a: expr, $b: expr, $d: expr) => {
        test_tools::assert_approx_equals($a, $b, $d);
    };
    ($a: expr, $b: expr) => {
        test_tools::assert_approx_equals($a, $b, test_tools::TEST_DELTA);
    };
}

pub(crate) use assert_approx_eq;