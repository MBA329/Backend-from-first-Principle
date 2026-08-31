// proto:  rpc Chat(stream ChatMessage) returns (stream ChatMessage);
import * as grpc from '@grpc/grpc-js';

// ===== SERVER: loop on('data') and call write() on the same stream =====
class UserServer {
    chat(call: grpc.ServerDuplexStream<any, any>) {
        call.on('data', (msg: any) => {
            // echo back (or broadcast to a room, etc.), can write anytime, any number
            const reply = new ChatMessage();
            reply.setUser("server");
            reply.setText("ack: " + msg.getText());
            call.write(reply);
        });
        
        call.on('end', () => {
            // client closed its send side
            call.end();
        });
    }
}

// ===== CLIENT: typically write concurrently, read via events =====
function chat(client: any) {
    const stream = client.chat();
    
    // concurrent receiver
    stream.on('data', (inMsg: any) => {
        console.log(`<< ${inMsg.getText()}`);
    });
    
    stream.on('end', () => {
        // server done
    });
    
    stream.on('error', (err: any) => {
        console.error(err);
    });
    
    for (const t of ["hi", "how are you", "bye"]) {
        const msg = new ChatMessage();
        msg.setUser("ada");
        msg.setText(t);
        stream.write(msg); // send concurrently
    }
    
    stream.end(); // signal we're done sending; receiver drains the rest
}

class ChatMessage {
    setUser(u: string){}
    setText(t: string){}
    getText(){ return ""; }
}
