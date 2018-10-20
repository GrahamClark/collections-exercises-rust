pub fn mean(v: &[i32]) -> f32 {
    v.iter().sum::<i32>() as f32 / v.len() as f32
}
