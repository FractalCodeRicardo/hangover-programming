#version 330

out vec4 color;
uniform float screenSize;
uniform float zoom;

float distance(vec3 p) {
  int max_iter = 10;
  int i = 0;
  float radius = 0.0;
  vec3 zn = vec3(p);
  float d = 1.0;

  while (i < max_iter) {
    radius = length(zn);

    if (radius > 2) {
      break;
    }

    float theta = atan(length(zn.xy), zn.z);
    float phi = atan(zn.y, zn.x);
    float r = pow(radius, 8);

    d = 8.0 * pow(radius, 7.0) * d + 1.0;

    float sint = sin(theta * 8);
    zn.x = r * sint * cos(phi * 8);
    zn.y = r * sint * sin(phi * 8);
    zn.z = r * cos(theta * 8);

    zn += p;

    i += 1;
  }

  return 0.5 * log(radius) * radius / d;
}

vec3 march(vec3 origin, vec3 dir) {
  int max_iter = 100;
  int i = 0;
  float depth = 0.0;
  vec3 ray;

  while (i < max_iter) {
    ray = origin + depth * dir;

    float d = distance(ray);

    if (d < 0.002)
      break;

    depth += d;

    i += 1;
  }

  return ray;
}

vec3 getNormal(vec3 p) {
  float e = 0.001;

  vec3 n = vec3(
      distance(p + vec3(e, 0, 0)) - distance(p - vec3(e, 0, 0)),
      distance(p + vec3(0, e, 0)) - distance(p - vec3(0, e, 0)),
      distance(p + vec3(0, 0, e)) - distance(p - vec3(0, 0, e))
    );

  return normalize(n);
}

void main() {
  vec2 uv = gl_FragCoord.xy;
  uv = uv - (0.5 * screenSize);
  uv = uv / screenSize;

  vec3 camera = vec3(0, 0, 3 + zoom);
  vec3 target = vec3(uv, 0);
  vec3 dir = target - camera;

  dir = normalize(dir);

  vec3 p = march(camera, dir);
  vec3 normal = getNormal(p);

  vec3 lightDir = normalize(vec3(-1.0, 1.0, 1.0));

  float diffuse = max(dot(normal, lightDir), 0.0);

  float brightness = 0.15 + diffuse * 0.85;

  color = vec4(vec3(brightness), 1.0);
}
