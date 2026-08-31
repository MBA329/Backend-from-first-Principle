pub fn percentile(latencies: &[std::time::Duration], p: f64) -> std::time::Duration {
    if latencies.is_empty() {
        return std::time::Duration::from_millis(0);
    }
    let mut sorted = latencies.to_vec();
    sorted.sort();
    let rank = (p / 100.0) * (sorted.len() - 1) as f64;
    let idx = rank.ceil() as usize;
    sorted[idx]
}

fn main() {
    let samples = vec![
        std::time::Duration::from_millis(12), std::time::Duration::from_millis(15),
        std::time::Duration::from_millis(22), std::time::Duration::from_millis(48),
        std::time::Duration::from_millis(95), std::time::Duration::from_millis(110),
        std::time::Duration::from_millis(340), std::time::Duration::from_millis(890),
        std::time::Duration::from_millis(1200), std::time::Duration::from_millis(4800),
    ];
    println!("P50: {:?}", percentile(&samples, 50.0));
    println!("P90: {:?}", percentile(&samples, 90.0));
    println!("P99: {:?}", percentile(&samples, 99.0));
}
