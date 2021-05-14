# Abstract
Traci is a ray tracer written in Rust that supports . In the final project, we added
CUDA support for increased performance; the parallelized features include: shadows, reflection,
refraction, and Bounding-Volumes Hierarchy (BVH).
 We also added & fine tuned super-resolution upsampling.

# Resources


# Implementation

## Hardware Specifications
- **CPU**—Intel i7-10750H (6 cores, base 2.6 GHz)
- **GPU**—RTX 2070 Max Q
- **Memory**—16 GB DDR4 RAM, 8 GB GDDR6

## Programming
Traci is primarially written in Rust

## CUDA Parallelization
In order to speed up the renderer and allow for high-framerate animations, our group took advantage of the CUDA parallelization API. Several features were ported over to the CUDA ray-tracer fork: shadows, bounding-volume hierarchies (BVH), texturing, and transforms.

<div style="width:100%;height:0px;position:relative;padding-bottom:100.000%;"><iframe src="https://streamable.com/e/7806t5" frameborder="0" width="100%" height="100%" allowfullscreen style="width:100%;height:100%;position:absolute;left:0px;top:0px;overflow:hidden;"></iframe></div>

| Resolution  | Rays per Pixel | Framerate |
| ----------- | -------------- | --------- |
| 1000 x 1000 | 1              | 115 FPS   |

## CUDA Shadows



<img src="{{site.baseurl}}/assets/img/shadow.png">

| Resolution  | Rays per Pixel | Rendering Time |
| ----------- | -------------- | -------------- |
| 1000 x 1000 | 2              | 1.8 sec        |

## CUDA Texturing

<div style="width:100%;height:0px;position:relative;padding-bottom:100.000%;"><iframe src="https://streamable.com/e/tcc48a" frameborder="0" width="100%" height="100%" allowfullscreen style="width:100%;height:100%;position:absolute;left:0px;top:0px;overflow:hidden;"></iframe></div>

| Resolution  | Rays per Pixel | Rendering Time |
| ----------- | -------------- | -------------- |
| 500 x 500   | 2              | 29 FPS         |

## AI Denoising + Supersampling

<img src="{{site.baseurl}}/assets/img/target.png">

<img src="{{site.baseurl}}/assets/img/predicted.png">

