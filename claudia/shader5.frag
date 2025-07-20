#ifdef GL_ES
precision mediump float;
#endif

uniform sampler2D tex0;
uniform vec2 resolution;
uniform float time;

varying vec2 vTexCoord;

// Simple vignette function
float vignette(vec2 uv) {
  vec2 position = uv - 0.5;
  float len = length(position);
  return smoothstep(0.8, 0.4, len);
}

void main() {
  // Flip Y to correct texture orientation
  vec2 uv = vec2(vTexCoord.x, 1.0 - vTexCoord.y);

  vec4 color = texture2D(tex0, uv);

  // Apply warm tint
  color.rgb = mix(color.rgb, vec3(1.0, 0.8, 0.6), 0.15);

  // Increase contrast
  color.rgb = pow(color.rgb, vec3(0.9));

  // Apply vignette
  float vig = vignette(vTexCoord);
  color.rgb *= vig;

  gl_FragColor = color;
}

