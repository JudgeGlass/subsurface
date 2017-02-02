#version 330 core

in ivec4 position;
in vec2 uv;

smooth out vec2 vuv;
smooth out float brightness_factor;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    gl_Position =  projection * view * model * vec4(position.xyz, 1.0);
    vuv = uv;

    float sun_percentage = ((position.w >> 4) & 0xF) / 16.0f;
    float block_percentage = (position.w & 0xF) / 16.0f;
    brightness_factor = pow(clamp((sun_percentage + block_percentage) * 0.9 + 0.1, 0.0, 1.0), 2.2);
}
