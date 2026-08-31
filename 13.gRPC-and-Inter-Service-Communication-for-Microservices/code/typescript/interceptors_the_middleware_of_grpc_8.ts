import * as grpc from '@grpc/grpc-js';

// Node.js gRPC handles interceptors quite differently from Go. 
// However, in @grpc/grpc-js there is interceptor support for clients, 
// and middleware-like patterns for servers (often through wrappers).
// We'll show a conceptual interceptor wrapper for a server method, 
// or how client interceptors work.

// A unary server interceptor concept:
function authInterceptor(
    call: grpc.ServerUnaryCall<any, any>,
    callback: grpc.sendUnaryData<any>,
    next: Function
) {
    const md = call.metadata;
    const tokens = md.get("authorization");
    
    if (tokens.length === 0 || !valid(tokens[0])) {
        return callback({
            code: grpc.status.UNAUTHENTICATED,
            message: "missing or invalid token",
            name: "Error"
        }, null);
    }
    
    // attach the verified identity for the handler to read from call
    (call as any).user = parse(tokens[0]);
    
    // call the next link / the real handler
    next(call, callback);
}

function loggingInterceptor(
    call: grpc.ServerUnaryCall<any, any>,
    callback: grpc.sendUnaryData<any>,
    next: Function
) {
    const start = Date.now();
    
    // intercept the callback
    const wrappedCallback = (err: any, value: any) => {
        const duration = Date.now() - start;
        console.log(`Call took ${duration}ms -> code: ${err ? err.code : 0}`);
        callback(err, value);
    };
    
    next(call, wrappedCallback);
}

// In practice, you wrap your handlers when building the server:
// Register the chain when building the server (outermost listed first):
function applyInterceptors(handler: Function, ...interceptors: Function[]) {
    return interceptors.reduceRight(
        (next, interceptor) => (call: any, cb: any) => interceptor(call, cb, next),
        handler
    );
}

// Example of binding it
// server.addService(UserServiceService, {
//    getUser: applyInterceptors(realGetUser, loggingInterceptor, authInterceptor)
// });

// Dummies
function valid(t: any): boolean { return true; }
function parse(t: any): any { return {}; }
