pub fn entropy(array: &Vec<u8>) -> f64 {
    let mut bincount = vec![0; 256];
    for a in array {
        bincount[*a as usize] += 1;
    }
    let total = array.len() as u32; // add 256 for smoothing?
    bincount
        .into_iter()
        .filter(|v| *v != 0)
        .map(|v| {
            let p = v as f64 / total as f64;
            -p * p.log2()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let array = (0..255).collect();
        println!("{:?}", entropy(&array));
        assert!((entropy(&array) - 8.0).abs() < 0.01)
    }
}
