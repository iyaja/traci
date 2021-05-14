# Abstract
Traci is a ray tracer written in Rust that supports . In the final project, we added
CUDA support for increased performance; the parallelized features include: shadows, reflection, and Bounding-Volumes Hierarchy (BVH).
 We also added & fine tuned super-resolution upsampling.

# Resources


# Implementation

## Hardware Specifications
- **CPU**—Intel i7-10750H (6 cores, base 2.6 GHz)
- **GPU**—RTX 2070 Max Q
- **Memory**—16 GB DDR4 RAM, 8 GB GDDR6
- **OS**—Linux 5.10.30-1-MANJARO x86_64

## Programming
For our final submission, we integrated new features into two of our group members' ray
tracers. Our first ray tracer, Traci, was implemented in Rust. We also used a Python script
to perform the AI supersampling on Traci's output. The second ray tracer was used for CUDA
parallelization, and was written in C.

## CUDA Parallelization
In order to speed up the renderer and allow for high-framerate animations, our group took advantage of the CUDA parallelization API. Several features were ported over to the CUDA ray-tracer fork: shadows, bounding-volume hierarchies (BVH), texturing, and transforms.

Shown below is a basic animation highlighting the performance benefits of rewriting the renderer in CUDA. In a basic scene, the parelellization enabled our renderer to output a frame in only 8 ms, allowing for fluid, high-resolution animations. To move the sphere, a transform system was written in to allow for easy re-positioning and rotation of the scene objects. Observing the specular highlight on the sphere, the real-time lighting effects are still preserved in the scene when animated in parallel.

All of the CUDA renders were performed with a block size of 32x32 threads.

<div style="width:100%;height:0px;position:relative;padding-bottom:100.000%;"><iframe src="https://streamable.com/e/7806t5" frameborder="0" width="100%" height="100%" allowfullscreen style="width:100%;height:100%;position:absolute;left:0px;top:0px;overflow:hidden;"></iframe></div>

| Resolution  | Rays per Pixel | Framerate |
| ----------- | -------------- | --------- |
| 1000 x 1000 | 1              | 115 FPS   |

## CUDA Shadows
Shadows typically add a fair bit of time to a render, so we also made shadow computation parallel with CUDA. The image is identitical to one rendered serially, but the render of the dragon mesh—with 871,414 triangles—took only a second to render. That represents a speedup of more than eight times from the serial renderer, which took 8.615 seconds to render the same scene.

Our group added additional optimizations while rendering the shadows, which explains why mesh + shadow render time was faster than that of the normal BVH. The implementation was able to reuse the intersection kernel for shadow intersection with very few modifications

<img src="{{site.baseurl}}/assets/img/shadow.png">

| Resolution  | Rays per Pixel | Rendering Time |
| ----------- | -------------- | -------------- |
| 1000 x 1000 | 2              | 1.0 sec        |

The below animation features the speedup from BVH & CUDA, parallelized shadows, and mesh transforms.

<div style="width:100%;height:0px;position:relative;padding-bottom:100.000%;"><iframe src="https://streamable.com/e/9v8j2a" frameborder="0" width="100%" height="100%" allowfullscreen style="width:100%;height:100%;position:absolute;left:0px;top:0px;overflow:hidden;"></iframe></div>

| Resolution  | Rays per Pixel | Framerate      |
| ----------- | -------------- | -------------- |
| 500 x 500   | 2              | 3 FPS          |

## CUDA Texturing

<div style="width:100%;height:0px;position:relative;padding-bottom:100.000%;"><iframe src="https://streamable.com/e/tcc48a" frameborder="0" width="100%" height="100%" allowfullscreen style="width:100%;height:100%;position:absolute;left:0px;top:0px;overflow:hidden;"></iframe></div>

| Resolution  | Rays per Pixel | Framerate      |
| ----------- | -------------- | -------------- |
| 500 x 500   | 2              | 29 FPS         |

## AI Denoising + Supersampling

<img src="{{site.baseurl}}/assets/img/target.png">

<img src="{{site.baseurl}}/assets/img/predicted.png">

