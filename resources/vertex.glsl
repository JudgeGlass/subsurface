#version 330 core

in ivec4 position;
in vec2 uv;

smooth out vec2 vuv;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    gl_Position =  projection * view * model * position;
    vuv = uv;
}
