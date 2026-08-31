import { WebSocketServer } from 'ws';
import * as http from 'http';

const server = http.createServer();
const wss = new WebSocketServer({ noServer: true });

async function verifyJWT(token: string): Promise<any> {
    // your auth logic (auth chapter)
    if (token === "valid") return { id: 1, name: "User" };
    throw new Error("Invalid token");
}

server.on('upgrade', async (request, socket, head) => {
    try {
        const url = new URL(request.url || '', `http://${request.headers.host}`);
        // AUTHENTICATE FIRST , before upgrading. Reject with a normal HTTP error.
        const token = url.searchParams.get("token") || ''; // or read a cookie / subprotocol
        
        const user = await verifyJWT(token);
        
        // Only now perform the upgrade; attach the identity to the connection.
        wss.handleUpgrade(request, socket, head, (ws) => {
            wss.emit('connection', ws, request, user);
        });
    } catch (err) {
        // 401, no upgrade
        socket.write('HTTP/1.1 401 Unauthorized\r\n\r\n');
        socket.destroy();
    }
});

wss.on('connection', (ws, request, user) => {
    // every message from this conn is now tied to `user`
    serve(ws, user);
});

function serve(ws: any, user: any) {
    ws.on('message', (msg: any) => {
        console.log(`Message from ${user.name}: ${msg}`);
    });
}
