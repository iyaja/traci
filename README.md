# Final Project: Ray Tracing

![A Sample Rendered Image](images/final.png)

## Features

[x] Commented code
[x] Ray-Plane Intersection
[x] Ray-Sphere Intersection
[x] Ray-Triangle Intersection
[x] Movable Camera
[ ] Orthographic and Perspective Projection
[ ] Multi-Jittered Sampling
[x] Simple Diffuse Shading
[x] Hard Shadows

## Build from Source

Traci is written in Rust. To compile to an executable, you first need a working installation of the Rust and Cargo. PLease visit [the official Rust website]() for instructions.

Then, to compile (with dependancies for math libraries, etc.) simply run:

```
cargo run --release
```

from the root of this project directory. This will fetch all dependancies, compile, and run the resulting binary (which is saved to `target/release/traci`). The output image is saved to `images/test.png`.

The sample image shown on this README was rendered at full HD resolution with 1000 samples per pixel on an 8 core CPU. Rendering took just under 10 minutes.

## Performance and Creative Features

The ray tracer implements the functionality in the project specification along with the following additional features:

- Multi-core parallelism for faster rendering
- Translucent multi-colored balls (i.e. refract light but are also colored)