import * as grpc from '@grpc/grpc-js';
import * as fs from 'fs';

// ===== SERVER over TLS =====
// const serverCerts = fs.readFileSync('server.crt');
// const serverKey = fs.readFileSync('server.key');
// const creds = grpc.ServerCredentials.createSsl(null, [{
//     cert_chain: serverCerts,
//     private_key: serverKey
// }]);

// every connection is now encrypted
// const server = new grpc.Server();
// server.bindAsync('0.0.0.0:443', creds, ...);

// ===== CLIENT over TLS =====
// const rootCerts = fs.readFileSync('ca.crt');
// const tlsCreds = grpc.credentials.createSsl(rootCerts); // trust this CA

const tlsCreds = grpc.credentials.createInsecure(); // Dummy for compilation
const client = new UserServiceClient('api.example.com:443', tlsCreds);

// ===== Per-call token (combine with TLS) =====
// Implement CallCredentials so a fresh token rides on every call:
const token = "my-secret-token";
const tokenCreds = grpc.credentials.createFromMetadataGenerator((args, cb) => {
    const md = new grpc.Metadata();
    md.add('authorization', 'Bearer ' + token);
    cb(null, md);
});

// combine transport and per-call credentials
const combinedCreds = grpc.credentials.combineChannelCredentials(
    tlsCreds,
    tokenCreds
);

const clientWithToken = new UserServiceClient('api.example.com:443', combinedCreds);

class UserServiceClient { constructor(a:any, b:any){} }
