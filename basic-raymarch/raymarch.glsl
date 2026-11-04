#version 330

out vec4 color;
uniform float screenSize;

float distanceSphere(vec3 p)
{
  return length(p) - 2; // radius
}

float march(vec3 origin, vec3 dir) {
  float max_iter = 100;
  int i = 0;
  float depth = 0.0;

  while (i < max_iter) {
    vec3 ray = origin + depth * dir;
    float d = distanceSphere(ray);

    depth += d;

    if (d < 0.001)
      return depth;

    i += 1;
  }
  return -1.0;
}

void main() {
  vec2 uv = (gl_FragCoord.xy - 0.5 * screenSize) / screenSize;

  vec3 origin = vec3(0, 0, 7);

  vec3 dir = vec3(uv, -1);
  dir = normalize(dir);

  float d = march(origin, dir); // distance to sphere

  if (d < 0.0) {
    color = vec4(0.0, 0.0, 0.0, 1.0);
  } else {
    float brightness = 1.0 - (d / 7.0);
    brightness = clamp(brightness, 0.0, 1.0);
    color = vec4(vec3(brightness), 1.0);
  }
}
