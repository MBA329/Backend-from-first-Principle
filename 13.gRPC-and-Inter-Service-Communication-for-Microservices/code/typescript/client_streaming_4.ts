// proto:  rpc UploadEvents(stream Event) returns (UploadSummary);
import * as grpc from '@grpc/grpc-js';

// ===== SERVER: on('data') in a loop, then callback() once at the end =====
class UserServer {
    uploadEvents(call: grpc.ServerReadableStream<any, any>, callback: grpc.sendUnaryData<any>) {
        let count = 0;
        
        call.on('data', (ev: any) => {
            store(ev);
            count++;
        });
        
        call.on('end', () => {
            // client finished sending, now reply once
            const summary = new UploadSummary();
            summary.setReceived(count);
            callback(null, summary);
        });
        
        call.on('error', (err) => {
            callback(err, null);
        });
    }
}

// ===== CLIENT: write() many, then end() to await the single reply =====
function uploadEvents(client: any, events: any[]) {
    const deadline = new Date();
    deadline.setSeconds(deadline.getSeconds() + 30);
    
    const stream = client.uploadEvents({ deadline }, (err: any, summary: any) => {
        // close our side, await the summary
        if (err) {
            console.error(err);
            return;
        }
        console.log(`server received ${summary.getReceived()} events`);
    });
    
    for (const ev of events) {
        stream.write(ev); // push each one up
    }
    
    stream.end(); // signal we're done sending
}

// Dummy definitions
class UploadSummary { setReceived(c: number){} getReceived(){ return 0; } }
function store(e: any){}
