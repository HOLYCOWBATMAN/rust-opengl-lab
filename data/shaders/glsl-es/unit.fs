#version 140

#ifdef GLES2
    precision mediump float;
#endif

void main()
{
    gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
}