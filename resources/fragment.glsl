#version 330 core

smooth in vec2 vuv;
smooth in float brightness_factor;
out vec4 ocolor;

uniform sampler2D block_texture;

void main() {
     ocolor = texture(block_texture, vuv) * brightness_factor;
}
