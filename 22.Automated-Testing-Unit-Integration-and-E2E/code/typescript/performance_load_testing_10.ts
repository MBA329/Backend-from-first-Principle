// For performance testing in TS/JS one might use autocannon or k6
import autocannon from 'autocannon';

async function runBenchmark() {
    const result = await autocannon({
        url: 'http://localhost:3000',
        connections: 10,
        pipelining: 1,
        duration: 10
    });
    console.log(result);
}
