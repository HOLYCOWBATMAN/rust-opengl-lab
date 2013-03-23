#version 150
in vec2 Texcoord;
out vec4 outColor;
uniform vec3 inColor;
uniform sampler2D tex;
void main() {
    outColor = texture(tex, Texcoord) * vec4(inColor, 1.0);
}