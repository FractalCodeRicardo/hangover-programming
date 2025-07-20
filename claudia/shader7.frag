#ifdef GL_ES
precision mediump float;
#endif

uniform sampler2D tex0;
uniform vec2 resolution;
uniform float time;

varying vec2 vTexCoord;

void main() {
    vec2 uv = vTexCoord;

    // Gentle wave distortion
    float wave = sin(uv.y * 30.0 + time * 5.0) * 0.01;
    uv.x += wave;

    vec4 originalColor = texture2D(tex0, uv);

    // Trippy color shift
    float r = originalColor.r + 0.1 * sin(time + uv.y * 10.0);
    float g = originalColor.g + 0.1 * cos(time + uv.x * 10.0);
    float b = originalColor.b + 0.1 * sin(time + uv.x * uv.y * 10.0);

    // Blend 70% original, 30% effect
    vec4 trippyColor = vec4(r, g, b, 1.0);
    gl_FragColor = mix(originalColor, trippyColor, 0.3);
}
