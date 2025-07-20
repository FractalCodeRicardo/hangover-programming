
#ifdef GL_ES
precision mediump float;
#endif

uniform sampler2D tex0;
uniform float u_time;
varying vec2 vTexCoord;

// Recursive tiling
vec2 fractalCoords(vec2 uv, int levels) {
  for (int i = 0; i < 10; i++) {
    if (i >= levels) break;
    uv = abs(fract(uv * 2.0) - 0.5);
  }
  return uv;
}

void main() {
  vec2 uv = vTexCoord;

  // Flip Y for p5.js compatibility
  uv.y = 1.0 - uv.y;

  // Animate the fractal levels
  int levels = 5 + int(mod(u_time * 0.5, 5.0));

  vec2 coord = fractalCoords(uv, levels);
  vec4 color = texture2D(tex0, coord);

  gl_FragColor = color;
}
