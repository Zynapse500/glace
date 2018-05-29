#version 330

attribute vec3 position;
attribute vec4 color;

uniform Transform {
    mat4 projection;
    mat4 view;
};

out FragData {
    vec3 position;
    vec4 color;
} frag;

void main() {
	gl_Position = projection * view * vec4(position, 1.0);
//    gl_Position = vec4(position, 1.0);

	frag.position = position;
	frag.color = color;
}
