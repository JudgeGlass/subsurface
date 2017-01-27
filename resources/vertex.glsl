#version 330 core

in vec4 position;
in vec4 color;

out vec4 vcolor;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    gl_Position =  projection * view * model * position;
    vcolor = color;
}
