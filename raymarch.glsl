#version 330

out vec4 color;
uniform float screenSize;

float distanceSphere(vec3 p) {
  return length(p) - 2;
}

float march(vec3 origin, vec3 dir) {
  int max_iter = 100;
  int i = 0;
  float depth = 0.0;

  while (i < 100) {
    vec3 ray = origin + depth * dir;

    float d = distanceSphere(ray);

    if (d < 0.001)
      return depth;

    depth += d;

    i += 1;
  }

  return -1.0;
}

void main() {
  vec2 pixel = gl_FragCoord.xy;
  pixel = pixel - (0.5 * screenSize);
  pixel = pixel / screenSize;

  vec3 origin = vec3(0, 0, 7);
  vec3 dir = vec3(pixel, -1);
  dir = normalize(dir);

  float d = march(origin, dir);

  if (d < 0.0)
    color = vec4(0, 0, 0, 1);
  else {
    float b = 1.0 - (d / 7.0);
    b = clamp(b, 0.0, 1.0);
    color = vec4(b, b + 0.1, b, 1.0);
  }
}
