#ifdef GL_ES
precision mediump float;
#endif

uniform sampler2D tex0;
uniform vec2 resolution;
uniform float time;

varying vec2 vTexCoord;

// Vignette with slow pulse
float vignette(vec2 uv) {
  vec2 pos = uv - 0.5;
  float len = length(pos);
  
  // Pulse between 0.4 and 0.7 radius slowly
  float pulse = 0.55 + 0.15 * sin(time * 0.3); // slower speed here (0.3)
  
  return smoothstep(pulse + 0.1, pulse - 0.1, len);
}

void main() {
  vec2 uv = vec2(vTexCoord.x, 1.0 - vTexCoord.y);

  vec4 color = texture2D(tex0, uv);

  // Brighten image
  color.rgb = color.rgb * 1.2;

  // Add blue tint
  color.rgb = mix(color.rgb, vec3(0.6, 0.8, 1.0), 0.3);

  // Slightly increase contrast
  color.rgb = pow(color.rgb, vec3(0.95));

  // Vignette with pulse
  float vig = vignette(vTexCoord);
  color.rgb *= vig;

  gl_FragColor = color;
}
