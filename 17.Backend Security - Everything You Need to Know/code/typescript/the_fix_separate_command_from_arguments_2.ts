import { exec, spawn } from "child_process";

const userFilename = "some_input"; // mock

// X VULNERABLE, passes through shell interpreter
exec("ffmpeg -i input.jpg -o " + userFilename);

// OK SAFE, command and each argument are separate params
// Shell never sees userFilename, it goes straight to the process
const cmd = spawn(
    "ffmpeg",
    [
        "-i", "input.jpg",
        "-vf", "scale=800:600",
        userFilename    // treated as a string argument, not shell code
    ]
);
cmd.stdout.on("data", (data) => console.log(data));
