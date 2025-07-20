#ifdef GL_ES
precision mediump float;
#endif

uniform sampler2D tex0;
varying vec2 vTexCoord;

void main() {
  vec4 color = texture2D(tex0, vTexCoord);
  
  // Convert to grayscale using luminance method
  float gray = dot(color.rgb, vec3(0.299, 0.587, 0.114));
  
  gl_FragColor = vec4(vec3(gray), color.a);
}
