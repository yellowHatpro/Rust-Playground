pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub fn subtract(left: i32, right: i32) -> i32 {
    left - right
}
pub fn multiply(left: i32, right: i32) -> i64 {
    (left*right) as i64
}
pub fn divide(left: i32, right: i32) -> Result<f32, &'static str> {
    if right == 0 {
        return Err("Division by 0");
    }
    Ok((left/right )as f32)
}
pub fn pow(base: i32, exp: i32) -> i32 {
    if exp==0 {
        return 1;
    }
    return base*pow (base,exp-1);
}


