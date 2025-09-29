pub fn sum(v: impl Iterator<Item = i64>) -> i64 { 
  return v.sum(); 
}

pub fn product(v: impl Iterator<Item = i64>) -> i64 { 
  return v.product(); 
}

pub fn quocient(v: impl Iterator<Item = i64>) -> i64 {
  return v.reduce(|acc, n| acc / n).unwrap_or(0);
}

pub fn difference(v: impl Iterator<Item = i64>) -> i64 {
  return v.reduce(|acc, n| acc - n).unwrap_or(0);
}