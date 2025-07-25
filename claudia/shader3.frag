
#ifdef GL_ES
precision mediump float;
#endif

uniform float time;
uniform vec2 mouse;
uniform vec2 resolution;

varying vec2 vTexCoord;

void main() {
  // Reconstruct pixel coordinates from vTexCoord
  vec2 fragCoord = vTexCoord * resolution;

  // Flip Y axis for p5.js
  fragCoord.y = resolution.y - fragCoord.y;

  vec2 position = (fragCoord / resolution) + mouse / 4.0;

  float color = 0.0;
  color += sin(position.x * cos(time / 15.0) * 80.0) + cos(position.y * cos(time / 15.0) * 10.0);
  color += sin(position.y * sin(time / 10.0) * 40.0) + cos(position.x * sin(time / 25.0) * 40.0);
  color += sin(position.x * sin(time / 5.0) * 10.0) + sin(position.y * sin(time / 35.0) * 80.0);
  color *= sin(time / 10.0) * 0.5;

  gl_FragColor = vec4(vec3(color, color * 0.5, sin(color + time / 3.0) * 0.75), 1.0);
}

