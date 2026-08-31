import http from 'http';
import express from 'express';

const app = express();

const server = http.createServer(app);

server.timeout = 5000;          // ReadTimeout
server.keepAliveTimeout = 60000; // IdleTimeout: how long to hold an idle keep-alive conn
server.headersTimeout = 2000;    // ReadHeaderTimeout: mitigates Slowloris (slow-header attacks)

server.listen(8080, () => {
    console.log('Server listening on port 8080');
});
