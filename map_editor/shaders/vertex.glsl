#version 460 core

layout (location = 0) in vec3 Positions;

void main() {
    gl_Position = vec4(Positions, 1.0);
}