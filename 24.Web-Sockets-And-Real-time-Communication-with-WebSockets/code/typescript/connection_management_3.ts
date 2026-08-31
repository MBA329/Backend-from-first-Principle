import { WebSocket } from 'ws';

class Client {
    ws: WebSocket;
    sendQueue: any[] = [];
    
    constructor(ws: WebSocket) {
        this.ws = ws;
    }
}

class Hub {
    clients: Set<Client> = new Set();

    register(client: Client) {
        this.clients.add(client);
    }

    unregister(client: Client) {
        this.clients.delete(client);
        client.ws.close();
    }

    broadcast(message: string) {
        for (const client of this.clients) {
            if (client.ws.readyState === WebSocket.OPEN) {
                // In Node.js, ws buffer sizes can be checked. 
                // buffer full -> slow client; drop it (sec 12)
                if (client.ws.bufferedAmount > 1024 * 1024) { // 1MB limit for example
                    this.unregister(client);
                } else {
                    client.ws.send(message);
                }
            }
        }
    }
}
