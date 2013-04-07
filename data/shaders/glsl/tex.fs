#version 150
in vec3 Color;
in vec2 Texcoord;

out vec4 outColor;

uniform float texAlpha;
uniform sampler2D texHuis;
uniform sampler2D texBanana;

void main() {
    outColor = mix(texture(texHuis, Texcoord), texture(texBanana, Texcoord), texAlpha);
}