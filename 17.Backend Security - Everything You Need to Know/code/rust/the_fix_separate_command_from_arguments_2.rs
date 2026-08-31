use std::process::Command;

fn main() -> std::io::Result<()> {
    let user_filename = "some_input"; // mock

    // X VULNERABLE, passes through shell interpreter
    let _cmd = Command::new("sh")
        .arg("-c")
        .arg(format!("ffmpeg -i input.jpg -o {}", user_filename))
        .output()?;

    // OK SAFE, command and each argument are separate params
    // Shell never sees userFilename, it goes straight to the process
    let _output = Command::new("ffmpeg")
        .arg("-i")
        .arg("input.jpg")
        .arg("-vf")
        .arg("scale=800:600")
        .arg(user_filename) // treated as a string argument, not shell code
        .output()?;

    Ok(())
}
