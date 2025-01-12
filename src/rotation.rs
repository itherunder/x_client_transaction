use crate::cubic_curve::Curve;

pub fn convert_rotation_to_matrix(degrees: Curve) -> Vec<f64> {
    let rad = radians(degrees);
    vec![rad.cos(), -rad.sin(), rad.sin(), rad.cos()]
}

pub fn radians(degrees: Curve) -> f64 {
    degrees.as_f64() * std::f64::consts::PI / 180.0
}
