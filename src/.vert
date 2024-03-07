#version 330 core

layout(location = 0) in vec2 Postition;

void main() {
    gl_Position = vec4(Postition, 0.0, 1.0);
}