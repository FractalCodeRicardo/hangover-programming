#version 330

out vec4 color;

uniform float bounds;
uniform float xoffset;
uniform float yoffset;

float getDistance(float x, float y) {
  int i = 0;
  float z = 0;
  float zi = 0;
  int max = 500;
  float bound = 2;
  float distance = 0.0;

  while(i < max) {

    // the formula is zn = zn-1 ^ 2 + c
    // z ^ 2
    
    float t = z;

    z = (z*z) - (zi * zi);
    zi = 2 * t * zi;

    // z ^ 2 + c
    z = z + x;
    zi = zi + y;

    distance = sqrt(z*z + zi*zi);
    if (distance > 2) {
      return distance;
    }

    i = i + 1;
  }


  return distance;
}

float getMandelCord(float pos) {
  float size = 800.0;
  float step = (bounds * 2) / 800;
  float newPos = pos * step - bounds;

  return newPos;
}


void main() {
  float x = getMandelCord(gl_FragCoord.x) + xoffset;
  float y = getMandelCord(gl_FragCoord.y) + yoffset;

  float d = getDistance(x, y);
  if (d < 2) {
    float c = d/2.0;
    color = vec4(0.0, 0.5 + sin(c), 0.4 + sin(c), 1.0);
  } else {
    color = vec4(0.0, 0.0, 0.0, 1.0);
  }
}
