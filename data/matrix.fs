#ifdef GLES2
    precision mediump float;
#endif

varying vec2 vTextureCoord;

uniform sampler2DRect uSampler;

void main(void) {
    gl_FragColor = texture2DRect(uSampler, vec2(vTextureCoord.s, vTextureCoord.t));
}