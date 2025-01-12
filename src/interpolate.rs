use crate::cubic_curve::Curve;

pub fn interpolate(from_list: Vec<Curve>, to_list: Vec<Curve>, f: Curve) -> Vec<Curve> {
    if from_list.len() != to_list.len() {
        panic!("Mismatched interpolation arguments {from_list:?}: {to_list:?}")
    }

    let mut out = Vec::new();
    for i in 0..from_list.len() {
        out.push(interpolate_num(from_list[i], to_list[i], f));
    }

    out
}

pub fn interpolate_num(from_val: Curve, to_val: Curve, f: Curve) -> Curve {
    from_val * (1 - f) + to_val * f
}
