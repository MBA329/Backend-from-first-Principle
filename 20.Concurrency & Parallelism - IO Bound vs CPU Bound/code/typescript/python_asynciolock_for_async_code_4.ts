// In JS, actor-like message passing isn't necessary for simple counters,
// but we can simulate a channel with an async queue or stream.
// Here we just use an async generator or a simple Event Emitter.

import { EventEmitter } from 'events';

const ch = new EventEmitter();
let counter = 0;

ch.on('delta', (d) => {
    counter += d;
});

for (let i = 0; i < 1000; i++) {
    ch.emit('delta', 1);
}

// Since emit is synchronous, it's already done
console.log(counter);
