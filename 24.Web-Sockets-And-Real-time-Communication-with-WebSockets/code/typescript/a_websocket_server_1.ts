import { WebSocketServer, WebSocket } from 'ws';
import * as http from 'http';

const server = http.createServer();
const wss = new WebSocketServer({ noServer: true });

server.on('upgrade', (request, socket, head) => {
    // CheckOrigin guards against cross-site hijacking , DO NOT leave it open (sec 14).
    const origin = request.headers.origin;
    if (origin !== "https://app.example.com") {
        socket.write('HTTP/1.1 401 Unauthorized\r\n\r\n');
        socket.destroy();
        return;
    }

    wss.handleUpgrade(request, socket, head, (ws) => {
        // performs the 101 handshake (sec 4)
        wss.emit('connection', ws, request);
    });
});

wss.on('connection', (ws: WebSocket) => {
    // the READ LOOP , one per connection
    ws.on('message', (message: Buffer, isBinary: boolean) => {
        // echo it straight back
        ws.send(message, { binary: isBinary });
    });

    ws.on('close', () => {
        // client closed or connection died (sec 6) -> exit, cleanup
    });
});

server.listen(8080, () => {
    console.log('Listening on port 8080');
});
