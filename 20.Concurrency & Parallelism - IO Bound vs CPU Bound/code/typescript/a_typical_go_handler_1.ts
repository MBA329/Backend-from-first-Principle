import net from 'net';

// Node.js is single-threaded, but handles connections asynchronously via the Event Loop
const server = net.createServer((conn) => {
    // For each connection, Node registers events rather than spawning a new thread
    handleConn(conn);
});

function handleConn(conn: net.Socket) {
    conn.on('data', (d) => { /* read */ });
}
