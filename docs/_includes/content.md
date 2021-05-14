# Team
- Ajay Uppili Arasanipalai (`aua2`)
- Rauf Makharov (`raufm2`)
- Patrick Gallagher (`pjg4`)
- Kevin Palani (`kevinrp2`)
- Sukrit Ganesh (`sukritg2`)

# Abstract
Traci is a ray tracer written in Rust that supports . In the final project, we added
CUDA support for increased performance; the parallelized features include: shadows, reflection, and Bounding-Volumes Hierarchy (BVH).
 We also added & fine tuned super-resolution upsampling.

# Resources

## CUDA Resources
**Basic CUDA Ray Tracing**

- [https://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html](https://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html)
- [https://developer.nvidia.com/blog/accelerated-ray-tracing-cuda/](https://developer.nvidia.com/blog/accelerated-ray-tracing-cuda/)
- [https://www.nvidia.com/content/nvision2008/tech_presentations/Game_Developer_Track/NVISION08-Interactive_Ray_Tracing.pdf](https://www.nvidia.com/content/nvision2008/tech_presentations/Game_Developer_Track/NVISION08-Interactive_Ray_Tracing.pdf)


**CUDA + BVH**
- [https://developer.nvidia.com/blog/thinking-parallel-part-ii-tree-traversal-gpu/](https://developer.nvidia.com/blog/thinking-parallel-part-ii-tree-traversal-gpu/)

**CUDA + Textures**
- [http://cuda-programming.blogspot.com/2013/02/texture-memory-in-cuda-what-is-texture.html](http://cuda-programming.blogspot.com/2013/02/texture-memory-in-cuda-what-is-texture.html)

## Supersampling Resources
[Pytorch model](https://github.com/xinntao/BasicSR)

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

# Added Features 

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

Although processing capability has increased exponentially over the last few decades, most graphics engines relying purely on ray tracing and other brute force techniques cannot realistically render a detailed scene at an adequate framerate. Furthermore, bugs in the rendering engine may result in artifacts appearing in the scene. Thankfully, artificial intelligence has enabled enginners to overcome performance limitations and increase the clarity of an image without pushing the rendering engine further. AI supersampling makes use of a neural network to remove noise and artifacts and increase the resolution of the image. We used a [pretrained Pytorch model](https://github.com/xinntao/BasicSR) from BasicSR, an open-source toolkit. We then wrote a bash script to feed the images from the ray tracer into this model; in the future, we would like to integrate the model with the Rust code. The model worked well on a pixelated image; the resulting supersampled image was similar to the target image and significantly better than the bilinearly upscaled image. The supersampling process took significantly less time than rendering a higher resolution image.

![Supersampling Example](https://cdn.discordapp.com/attachments/828897155388801035/842820580889591848/figure.png)

|                | Native Resolution | Target Resolution | Samples Per Pixel | Max Depth | Rays Cast     | Rendering Time          |
| -------------- | ----------------- | ----------------- | ----------------- | ----------| ------------- | ----------------------- |
No Supersampling | 480 x 270         | 480 x 270         | 1000              | 50        | 6,480,000,000 | 480s                    |
Supersampling    | 120 x 68          | 480 x 270         | 1000              | 50        | 408,000,000   | 30s (ray-tracing) + 2s (supersampling) |

# Conclusion
Overall, we were very satisfied with the results of this project. Implementing the ray-tracer in CUDA significantly sped up the computation, and the AI supersampling notably improved the quality of the image. The latter part of the project demonstrated the endless power of AI and how it can solve otherwise computationally intensive problems. We were especially impressed with the versatility of Rust, an innovative programming language which improves significantly on C++. Due to our flexible implementation, we can easily add more features to our ray tracer at a later time. In the future, we would like to integrate the supersampling with the ray tracer, so the user does not have to run a separate script to feed the output images from the Rust-based ray tracer into the python-based supersampler. We would also like to incorporate AI into other parts of our ray-tracer, as we believe there is a lot of potential for neural networks to enhance various aspects of the rendering image. Overall, this project was a great experience where we learned new skills, tested out new ideas, and built a modern application that demonstrated the power of modern computing.