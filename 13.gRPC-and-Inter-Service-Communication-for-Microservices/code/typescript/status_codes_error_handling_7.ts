import * as grpc from '@grpc/grpc-js';

// SERVER: return a typed status, not a bare error string.
class UserServer {
    getUser(
        call: grpc.ServerUnaryCall<GetUserRequest, GetUserResponse>,
        callback: grpc.sendUnaryData<GetUserResponse>
    ) {
        const req = call.request;
        if (!req.getId()) {
            return callback({
                code: grpc.status.INVALID_ARGUMENT,
                message: "id is required",
                name: 'Error'
            }, null);
        }
        
        const user = this.dbFind(req.getId());
        if (!user) {
            return callback({
                code: grpc.status.NOT_FOUND,
                message: `no user with id "${req.getId()}"`,
                name: 'Error'
            }, null);
        }
        
        const resp = new GetUserResponse();
        resp.setUser(user);
        callback(null, resp);
    }
    
    dbFind(id: string): any { return null; }
}

// CLIENT: inspect the code to decide what to do.
function clientCall(client: any, req: any) {
    client.getUser(req, (err: grpc.ServiceError, resp: any) => {
        if (err) {
            // pull the status out of the error (in TS, the error object IS the status)
            switch (err.code) {
                case grpc.status.NOT_FOUND:
                    // expected, show "not found" in UI
                    break;
                case grpc.status.UNAVAILABLE:
                    // transient, retry with backoff, sec 17
                    break;
                default:
                    console.log(`unexpected: ${err.code}: ${err.message}`);
            }
        }
    });
}

// Dummies
class GetUserRequest { getId(){ return ""; } }
class GetUserResponse { setUser(u:any){} }
