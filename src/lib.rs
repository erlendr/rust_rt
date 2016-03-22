pub fn magnitude(v: &Vec<f64>) -> f64 {
    let x = v[0];
    let y = v[1];
    let z = v[2];

    return ((x*x) + (y*y) + (z*z) as f64).sqrt();
}

pub fn normalize(v: &Vec<f64>) -> Vec<f64> {
    let magnitude = magnitude(v);
    return v.iter().map(|&x| x / magnitude).collect();
}

pub fn negative(v: &Vec<f64>) -> Vec<f64> {
    return v.iter().map(|&x| -x).collect();
}
