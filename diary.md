#setup

## summary

GLFW is used to talk to the OS.
stb_image is used to load images.
nvidia-texture-tools is used to preprocess those images into DX* compressed textures.

install glfw
define GLFW_INCLUDE_GLCOREARB if on osx so that opengl 3.2 is used

build stb_image

# Components

## stb_image
servo is using this binding and have plans to replace it with something faster.
Keep an eye on an implementation coming out of servo.

## model loader

TODO: Make a command line tool for converting .obj files into an interleaved stream of coords.

# notes
rust now uses absolute imports only
the prelude contains a minimal set of implicit includes

# reference

https://developer.apple.com/graphicsimaging/opengl/capabilities/

# tooling

nvidia-texture-tools (nvcompress)
    compress images from common formats such as png jpeg to dx* compressed formats

Shader Maker
    experiment with shaders

Instruments
    performance monitoring for OSX