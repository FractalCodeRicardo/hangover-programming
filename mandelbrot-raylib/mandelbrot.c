#include "raylib.h"
#include <math.h>

#define BUFFER 50000

const int WINDOW_SIZE = 800;
double BOUND = 2;
double STEP = 0.02;
double Y_OFFSET = 0.0;
double X_OFFSET = 0.0;

typedef struct  {
  double x;
  double y;
} Point;

typedef struct  {
  Point values[BUFFER];
  int size;
} Points;

Points points;

bool IsMandelbrot(double x, double y) {
  int i = 0;
  double z = 0;
  double zi = 0;
  int max = 500;

  while(i < max) {

    // the formula is zn = zn-1 ^ 2 + c
    // z ^ 2
    
    double t = z;

    z = (z*z) - (zi * zi);
    zi = 2 * t * zi;

    // z ^ 2 + c
    z = z + x;
    zi = zi + y;

    if (sqrt(z*z + zi*zi) > BOUND) {
      break;
    }

    i = i + 1;
  }


  return i < max;
}

Points GetPoints() {
  Points points;


  int i = 0;
  for(double x = -BOUND; x < BOUND; x = x + STEP) {
    for(double y = -BOUND; y < BOUND; y = y + STEP) {

      if (i >= BUFFER) {
        return points;
      }

      if (!IsMandelbrot(x, y)) {

        points.values[i] = (Point) {
          .x = X_OFFSET + BOUND + x, 
          .y = Y_OFFSET + BOUND + y
        };

        i = i + 1;

        points.size = i;
      }
      
    }
  }


  return points;
}

void DrawPoints() {
  double size = WINDOW_SIZE / (BOUND * 2);
  double cell_size = STEP * size;

  for(int i = 0; i < points.size ; i++) {
    Point p = points.values[i];

    int x = size * p.x;
    int y = size * p.y;

    DrawRectangle(x, y, cell_size, cell_size, WHITE );
  }
}

void HandleKeys() {

  if (IsKeyPressed(KEY_UP)) {
    Y_OFFSET = Y_OFFSET - 0.1;
    points = GetPoints();
  }

  if (IsKeyPressed(KEY_DOWN)) {
    Y_OFFSET = Y_OFFSET + 0.1;
    points = GetPoints();
  }

  if (IsKeyPressed(KEY_RIGHT)) {
    X_OFFSET = X_OFFSET  +0.1;
    points = GetPoints();
  }

  if (IsKeyPressed(KEY_LEFT)) {
    X_OFFSET = X_OFFSET - 0.1;
    points = GetPoints();
  }

  if (IsKeyPressed(KEY_H)) {
    STEP = STEP - 0.001;
    points = GetPoints();
  }

  if (IsKeyPressed(KEY_J)) {
    STEP = STEP + 0.001;
    points = GetPoints();
  }


  if (IsKeyPressed(KEY_K)) {
    BOUND = BOUND - 0.1;
    points = GetPoints();
  }

  if (IsKeyPressed(KEY_L)) {
    BOUND = BOUND + 0.1;
    points = GetPoints();
  }
}


int main() {
  
  points = GetPoints();
  InitWindow(WINDOW_SIZE, WINDOW_SIZE, "Mandelbrot");

  while(!WindowShouldClose()) {
    BeginDrawing();
    HandleKeys();
    ClearBackground(BLACK);
    DrawPoints();
    EndDrawing();
  }

  CloseWindow();
}
