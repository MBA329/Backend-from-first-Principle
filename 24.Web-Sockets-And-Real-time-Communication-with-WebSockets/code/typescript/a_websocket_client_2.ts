import WebSocket from 'ws';

function runClient() {
    const token = 'your-token';
    // Dial performs the upgrade handshake; header carries auth where allowed (sec 13).
    const ws = new WebSocket('wss://api.example.com/ws', {
        headers: {
            'Authorization': 'Bearer ' + token
        }
    });

    ws.on('open', () => {
        // send a message
        ws.send(JSON.stringify({ type: 'hello' }));
        
        // graceful close: send a close frame, then the socket shuts (sec 6)
        ws.close(1000, 'bye'); // 1000 is normal closure
    });

    ws.on('message', (data: Buffer) => {
        console.log(`recv: ${data}`);
    });

    ws.on('error', (err) => {
        console.error(err);
    });
}

runClient();
