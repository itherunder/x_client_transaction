pub fn convert_rotation_to_matrix(degrees: f64) -> Vec<f64> {
    let rad = radians(degrees);
    vec![rad.cos(), -rad.sin(), rad.sin(), rad.cos()]
}

pub fn radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}
