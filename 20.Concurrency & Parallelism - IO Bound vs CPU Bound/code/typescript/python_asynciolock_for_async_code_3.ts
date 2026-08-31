// In Node.js, JS execution is single-threaded, so we don't need Mutexes 
// for synchronous operations on shared memory.
let counter = 0;

async function increment() {
    counter++; // No race condition because JS is single-threaded
}

async function main() {
    const promises = [];
    for (let i = 0; i < 1000; i++) {
        promises.push(increment());
    }
    await Promise.all(promises);
    console.log(counter); // Always 1000
}
