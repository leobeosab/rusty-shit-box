attribute vec3 position;
uniform mat4 modelTransform;

void main() {
    gl_Position = modelTransform * vec4(position, 1.0);
}
