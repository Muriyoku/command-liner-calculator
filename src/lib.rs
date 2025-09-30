/// sum all values passed by, then return the total 
/// # Example 
/// ```
/// let values = vec![1.0, 2.0, 3.0].into_iter();
/// let result = command_line_calculator::sum(values);
/// 
/// assert_eq!(result, 6.0);
/// ```
pub fn sum(v: impl Iterator<Item = f64>) -> f64 { 
  return v.sum(); 
}
/// multiply all values passed by command line, then return the total. 
/// # Example 
/// ```
/// let values = vec![1.0, 1.0].into_iter();
/// let result = command_line_calculator::product(values);
/// 
/// assert_eq!(result, 1.0);
/// ```
pub fn product(v: impl Iterator<Item = f64>) -> f64 { 
  return v.product(); 
}
/// devide all numbers given, then return the quocient
/// # Example 
/// ```
/// let values = vec![10.0, 5.0].into_iter();
/// let result = command_line_calculator::quocient(values);
///  
/// assert_eq!(result, 2.0);
/// ```
pub fn quocient(v: impl Iterator<Item = f64>) -> f64 {
  return v.reduce(|acc, n| acc / n).unwrap_or(0.0);
}
/// subtract all numbers given then return the difference
/// # Example
/// ```
/// let values = vec![3.0, 1.0].into_iter();
/// let result = command_line_calculator::difference(values);
/// 
/// assert_eq!(result, 2.0);
/// ```
pub fn difference(v: impl Iterator<Item = f64>) -> f64 {
  return v.reduce(|acc, n| acc - n).unwrap_or(0.0);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_sum() {
    let v: std::vec::IntoIter<f64> = vec![1.0, 1.0].into_iter();
    assert_eq!(sum(v), 2.0);
  }
  
  #[test]
  fn it_multiply() {
    let v: std::vec::IntoIter<f64> = vec![1.0, 1.0].into_iter();
    assert_eq!(product(v), 1.0);
  }

  #[test]
  fn it_divide() {
    let v: std::vec::IntoIter<f64> = vec![10.0, 5.0].into_iter();
    assert_eq!(quocient(v), 2.0);
  } 

  #[test]
  fn it_subtract() {
    let v: std::vec::IntoIter<f64> = vec![3.0, 1.0].into_iter();
    assert_eq!(difference(v), 2.0);
  } 
}