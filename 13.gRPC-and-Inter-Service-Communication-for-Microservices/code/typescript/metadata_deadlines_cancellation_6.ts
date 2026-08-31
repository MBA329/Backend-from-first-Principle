import * as grpc from '@grpc/grpc-js';

// ===== CLIENT: attach a deadline + metadata =====
const deadline = new Date();
deadline.setSeconds(deadline.getSeconds() + 1); // absolute deadline

const meta = new grpc.Metadata();
const token = "my-token";
const reqID = "12345";
meta.add('authorization', 'Bearer ' + token);   // auth travels as metadata, sec 16
meta.add('x-request-id', reqID);              // trace id for correlation, like sec 15 of the layers manual

const client = new UserServiceClient('localhost:50051', grpc.credentials.createInsecure());
const req = new GetUserRequest();
req.setId("u42");

client.getUser(req, meta, { deadline }, (err: grpc.ServiceError, resp: any) => {
    if (err) {
        if (err.code === grpc.status.DEADLINE_EXCEEDED) {
            console.log("call timed out"); // a network-shaped failure a local call never had
        }
    }
});

// ===== SERVER: read metadata, respect the inherited deadline =====
class UserServer {
    getUser(
        call: grpc.ServerUnaryCall<GetUserRequest, GetUserResponse>,
        callback: grpc.sendUnaryData<GetUserResponse>
    ) {
        const md = call.metadata;
        const auth = md.get('authorization'); // verify token here (or in an interceptor, sec 15)

        // The gRPC Node library handles cancellation automatically. If the client 
        // cancels or times out, the call will emit a 'cancelled' event.
        if (call.cancelled) {
            return callback({ code: grpc.status.CANCELLED, message: 'Cancelled', name: 'Error' }, null);
        }
        
        call.on('cancelled', () => {
            // pass it straight to the DB driver so slow work is abandoned automatically
        });

        this.lookup(call.request.getId(), (err, user) => {
            if (call.cancelled) return;
            if (err) return callback(err, null);
            const resp = new GetUserResponse();
            resp.setUser(user);
            callback(null, resp);
        });
    }

    lookup(id: string, cb: (err: any, u: User) => void) { cb(null, new User()); }
}

// Dummy classes
class UserServiceClient { constructor(a:any,b:any){} getUser(a:any,b:any,c:any,d:any){} }
class GetUserRequest { setId(i:string){} getId(){ return ""; } }
class GetUserResponse { setUser(u:any){} }
class User {}
