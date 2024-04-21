#version 440
//precision mediump float;

layout(location = 0) in vec2 position;
layout(location = 1) in vec3 color;

out vec3 v_color;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    v_color = color;
}
