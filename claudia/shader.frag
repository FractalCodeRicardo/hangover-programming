#ifdef GL_ES
precision mediump float;
#endif

uniform sampler2D tex0;
varying vec2 vTexCoord;

void main() {
  // Flip Y axis of texture coordinates
  vec2 flippedCoord = vec2(vTexCoord.x, 1.0 - vTexCoord.y);
  gl_FragColor = texture2D(tex0, flippedCoord);
}
