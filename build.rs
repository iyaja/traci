use std::process::Command;

fn main() {
    // Note that there are a number of downsides to this approach
    let output = Command::new("make")
        .args(&["-C", "src/cuda/"])
        .output()
        .expect("failed to compile with CUDA support");

    println!("status: {}", output.status);
    println!("cargo:rerun-if-changed=src/cuda/");
}
