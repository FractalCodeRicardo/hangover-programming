#version 330

out vec4 color;
uniform float screenSize;
uniform float zoom;
uniform float angle;
vec3 rotateY(vec3 p, float angle) {
  float c = cos(angle);
  float s = sin(angle);

  return vec3(
    c * p.x - s * p.z,
    p.y,
    s * p.x + c * p.z
  );
}
float distance(vec3 p) {
  p = rotateY(p, angle);
  int max_iter = 16;
  int i = 0;
  float radius = 0.0;
  vec3 zn = vec3(p);
  float d = 1.0;
  float power = 8.0;

  while (i < max_iter) {
    radius = length(zn);

    if (radius > 1.5) {
      break;
    }

    float theta = atan(length(zn.xy), zn.z);
    float phi = atan(zn.y, zn.x);
    float r = pow(radius, power);

    d = pow(radius, power - 1) * power * d + 1.0;

    float sint = sin(theta * power);
    zn.x = r * sint * cos(phi * power);
    zn.y = r * sint * sin(phi * power);
    zn.z = r * cos(theta * power);

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
  vec2 uv = gl_FragCoord.xy / screenSize;
  uv = -1 + 2.0 * uv;

  // float radius = -2.5 + zoom;

  // float x = radius * sin(angle);
  // float z = radius * cos(angle);

  vec3 camera = vec3(0, 0, 2.5 + zoom);
  vec3 target = vec3(uv, 1.5);
  vec3 dir = target - camera;

  dir = normalize(dir);

  vec3 p = march(camera, dir);

  if (length(p) > 2.0) {
    color = vec4(0, 0, 0, 1);
    return;
  }

  vec3 normal = getNormal(p);

  vec3 lightDir = normalize(vec3(-1.0, 1.0, 1.0));

  // float diffuse = max(dot(normal, lightDir), 0.0);

  // float brightness = 0.15 + diffuse * 0.85;

  float diffuse = max(dot(normal, lightDir), 0.0);
  float brightness = 0.08 + diffuse * 0.92;

  color = vec4(vec3(brightness), 1.0);
}
