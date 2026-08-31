export function percentile(latencies: number[], p: number): number {
    if (latencies.length === 0) return 0;
    const sorted = [...latencies].sort((a, b) => a - b);
    const rank = (p / 100.0) * (sorted.length - 1);
    const idx = Math.ceil(rank);
    return sorted[idx];
}

const samples = [12, 15, 22, 48, 95, 110, 340, 890, 1200, 4800];
console.log("P50: ", percentile(samples, 50));
console.log("P90: ", percentile(samples, 90));
console.log("P99: ", percentile(samples, 99));
