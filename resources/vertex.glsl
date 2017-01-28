#version 330 core

in ivec4 position;
in vec4 color;
in vec2 uv;

out vec4 vcolor;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    gl_Position =  projection * view * model * position;
    vcolor = color;
}
