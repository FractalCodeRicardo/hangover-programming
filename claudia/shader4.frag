#ifdef GL_ES
precision mediump float;
#endif

uniform float time;
uniform vec2 resolution;
varying vec2 vTexCoord;

void main() {
    // Normalize coords to range [-1, 1] and fix aspect ratio
    vec2 uv = vTexCoord * 2.0 - 1.0;
    uv.x *= resolution.x / resolution.y;

    // Convert to polar coordinates
    float r = length(uv);
    float a = atan(uv.y, uv.x);

    // Animate the radial and angular values
    float pattern = sin(10.0 * r - time * 4.0) + cos(6.0 * a + time * 2.0);

    // Normalize to range [0, 1]
    float intensity = pattern * 0.5 + 0.5;

    // Use intensity to generate RGB
    vec3 color = vec3(
        intensity,
        0.5 * sin(time + intensity * 6.2831) + 0.5,
        1.0 - intensity
    );

    gl_FragColor = vec4(color, 1.0);
}
