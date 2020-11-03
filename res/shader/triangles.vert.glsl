#version 450

layout (location = 0) in vec2 vertex_position;
layout (location = 1) in vec2 vertex_uv;
layout (location = 2) in vec3 vertex_color;

layout (location = 0) out vec2 uv;
layout (location = 1) out vec3 color;

out gl_PerVertex {
	vec4 gl_Position;
};

void main() {
	uv = vertex_uv;
	color = vertex_color;
	float aspect = 128. / 72.;
	vec2 position_screen_space = vertex_position * 2. - 1.;
	gl_Position = vec4(position_screen_space, 0, 1);
}
