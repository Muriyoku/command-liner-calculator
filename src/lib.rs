pub fn sum(v: impl Iterator<Item = f64>) -> f64 { 
  return v.sum(); 
}

pub fn product(v: impl Iterator<Item = f64>) -> f64 { 
  return v.product(); 
}

pub fn quocient(v: impl Iterator<Item = f64>) -> f64 {
  return v.reduce(|acc, n| acc / n).unwrap_or(0.0);
}

pub fn difference(v: impl Iterator<Item = f64>) -> f64 {
  return v.reduce(|acc, n| acc - n).unwrap_or(0.0);
}