#version 330

attribute vec3 position;
attribute vec4 color;

uniform mat4 projection;
uniform mat4 view;

out FragData {
    vec3 position;
    vec4 color;
} frag;

void main() {
	gl_Position = projection * view * vec4(position, 1.0);

	frag.position = position;
	frag.color = color;
}
