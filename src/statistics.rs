use rustc_hash::FxHashMap;

pub fn entropy(array: &Vec<u8>) -> f64 {
    let mut bincount: FxHashMap<u8, u32> = FxHashMap::default();
    for a in array {
        *bincount.entry(*a).or_default() += 1;
    }
    let total = array.len() as u32; // add 256 for smoothing?
    bincount
        .values()
        .map(|v| {
            let p = (*v) as f64 / total as f64;
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
        assert!((entropy(&array) - 8.0).abs() < 0.01)
    }
}
