
# X VULNERABLE ,  shell=true sends everything through sh
subprocess.run(f"ffmpeg -i input.jpg -o {user_filename}", shell=true)

# OK SAFE ,  list form, shell=false (default)
subprocess.run([
    "ffmpeg", "-i", "input.jpg",
    "-vf", "scale=800:600",
    user_filename   # just a string, not interpreted
], check=true)
