#version 330

out vec4 color;
uniform float time;

float fixCoord(float coord) {
  float SIZE = 800;
  float RANGE = 0.5;

  float newCoord = coord - (SIZE/2);
  float proportion = (RANGE * 2) / SIZE;
  newCoord = newCoord * proportion;

  return newCoord;
}

int getIteration(float x, float y, int max) {
  int i = 0;

  // float cx = 0.356;
  // float cy = 0.356;


  // float cx = -0.7;
  // float cy = 0.27;

  // float cx = -0.8;
  // float cy = 0.156;
  
  // float cx = -0.123;
  // float cy = 0.745;
  
  float cx = -0.391;
  float cy = -0.587;

  float z = x;
  float zi = y;
  float d = 0;

  while(i < max) {

    // z^2 

    float t = z;

    z = (z*z) - (zi * zi);
    zi = 2 * t * zi;

    // z ^ 2 + c
    z = z + cx;
    zi = zi + cy;

    d = sqrt(z*z + zi*zi);

    if (d > 2) {
      return i;
    }

    i = i + 1;
  }

  return i;
}

void main() {
  float sx = gl_FragCoord.x;
  float sy = gl_FragCoord.y;

  sx = fixCoord(sx);
  sy = fixCoord(sy);

  int max =int(abs(sin(time * 0.5)) * 100);
  int i = getIteration(sx, sy, max);
  
  float t = i/max;

  vec4 green = vec4(1-t,1,1-t,t);
  vec4 blue = vec4(1-t,1-t,1,t);
  vec4 tcolor = vec4(t,t,t,1);

  color = mix(green, blue, 0.55);
}
