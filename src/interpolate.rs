pub fn interpolate(from_list: Vec<f64>, to_list: Vec<f64>, f: f64) -> Vec<f64> {
    if from_list.len() != to_list.len() {
        panic!("Mismatched interpolation arguments {from_list:?}: {to_list:?}")
    }

    let mut out = Vec::new();
    for i in 0..from_list.len() {
        out.push(interpolate_num(from_list[i], to_list[i], f));
    }

    out
}

pub fn interpolate_num(from_val: f64, to_val: f64, f: f64) -> f64 {
    from_val * (1.0 - f) + to_val * f
}
