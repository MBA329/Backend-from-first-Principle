// ===== SERVER =====
import * as grpc from '@grpc/grpc-js';
// Generated from user.proto (conceptual)
import { UserServiceService, IUserServiceServer } from './gen/user_grpc_pb';
import { GetUserRequest, GetUserResponse, User, Role } from './gen/user_pb';

// Implement the generated interface for forward-compat.
class UserServer implements IUserServiceServer {
    [name: string]: grpc.UntypedHandleCall;

    // The method signature is generated FROM the proto: call, callback
    getUser(
        call: grpc.ServerUnaryCall<GetUserRequest, GetUserResponse>,
        callback: grpc.sendUnaryData<GetUserResponse>
    ): void {
        const req = call.request;
        if (!req.getId()) {
            // typed error, sec 14
            return callback({
                code: grpc.status.INVALID_ARGUMENT,
                message: 'id is required'
            }, null);
        }
        
        // ...real work: query the DB by req.getId()...
        const u = new User();
        u.setId(req.getId());
        u.setEmail('ada@example.com');
        u.setFullName('Ada');
        u.setRole(Role.ROLE_ADMIN);
        
        const resp = new GetUserResponse();
        resp.setUser(u);
        
        callback(null, resp);
    }
}

function main() {
    const server = new grpc.Server();
    // wire the impl to the service
    server.addService(UserServiceService, new UserServer());
    server.bindAsync('0.0.0.0:50051', grpc.ServerCredentials.createInsecure(), () => {
        console.log('gRPC on :50051');
        server.start();
    });
}

// ===== CLIENT =====
import { UserServiceClient } from './gen/user_grpc_pb';

function callGetUser() {
    // insecure creds for local dev only
    const client = new UserServiceClient('localhost:50051', grpc.credentials.createInsecure());
    
    // always a deadline, sec 13
    const deadline = new Date();
    deadline.setSeconds(deadline.getSeconds() + 1);
    
    const req = new GetUserRequest();
    req.setId('u42');
    
    // looks local, runs remote
    client.getUser(req, { deadline }, (err, resp) => {
        if (err) {
            // err carries the gRPC status code
            console.error(`GetUser failed: ${err.message}`);
            return;
        }
        console.log(`got user: ${resp.getUser()?.getFullName()}`);
    });
}
