// proto: rpc ListUsers(ListUsersRequest) returns (stream User);
import * as grpc from '@grpc/grpc-js';

// ===== SERVER: receive one req, call stream.write(...) repeatedly =====
class UserServer {
    listUsers(call: grpc.ServerWritableStream<any, any>) {
        const filter = call.request.getFilter();
        const users = queryUsers(filter); // imagine this yields a big result set
        
        for (const u of users) {
            // push one message down the stream
            call.write(u);
        }
        
        // closes the stream cleanly (sends OK trailer)
        call.end();
    }
}

// ===== CLIENT: call once, then listen on 'data' and 'end' events =====
function listUsers(client: any) {
    const deadline = new Date();
    deadline.setSeconds(deadline.getSeconds() + 10);
    
    const req = new ListUsersRequest();
    req.setFilter("active");

    const stream = client.listUsers(req, { deadline });
    
    stream.on('data', (u: any) => {
        console.log(`user: ${u.getFullName()}`);
    });
    
    stream.on('end', () => {
        // server closed the stream, we're done
    });
    
    stream.on('error', (err: any) => {
        console.error(err);
    });
}

// Dummy definitions for types
class ListUsersRequest { setFilter(f: string){} getFilter(){} }
function queryUsers(f: any): any[] { return []; }
