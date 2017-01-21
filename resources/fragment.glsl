#version 330

in vec3 vcolor;
out vec4 ocolor;

void main() {
    ocolor = vec4(vcolor, 1.0);
}
