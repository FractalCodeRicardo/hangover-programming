#include "raylib.h"
#include <math.h>
#include <stdio.h>

#define BUFFER 100000
const double WINDOW_SIZE = 800;
double BOUND = 1.5;
double STEP = 0.02;
double Y_OFFSET = 0.0;
double X_OFFSET = 0.0;

typedef struct{
  double x;
  double y;
} p;

typedef struct {
   p values[BUFFER];
   int size;
} points;

points pts;

bool isMandelBrot(double x, double y) {
  int max = 500;
  double z = 0;
  double zi = 0;
  int i = 0;

  while( i < max) {

    double t = z;

    // z ^ 2
    z = (z*z) - (zi * zi);
    zi = 2  * t * zi;

    // z ^ 2 + c
    z = z + x;
    zi = zi + y;

    if (sqrt((z*z) + (zi*zi)) > BOUND) {
      break;
    }

    i = i + 1;
  }

  return i < max;
}

points getPoints() {
  points pts;
  double s = STEP;
  int i = 0;
  int limit = BUFFER;
  for (double x = -BOUND; x <= BOUND; x = x +s) {
    for (double y = -BOUND; y <= BOUND; y = y +s) {
      
      if (i >= limit) {
        break;
      }

      if (!isMandelBrot(x, y)) {
        pts.values[i] = (p) { X_OFFSET+ BOUND + x, Y_OFFSET+ BOUND + y};
        i = i + 1;
        pts.size = i;
      }

    }
  }

  printf("Points %i step %f \n", pts.size, STEP);

  return pts;
}

void drawPoints(points *pts) {
  double size = WINDOW_SIZE / (BOUND * 2); 
  double cell_size = size * STEP;

  

  for(int i = 0; i < pts->size; i++) {
    p p = pts->values[i];

    //printf("Point: %f %f \n", p.x, p.y);
    int x = p.x* size;
    int y = p.y* size;

    //printf("P %f %i, %f %i  Size  %f \n", p.x, x, p.y, y, size);
    DrawRectangle(x, y, cell_size, cell_size, WHITE);
  }
}

void HandleKeys() {

  if (IsKeyPressed(KEY_H)) {
    STEP = STEP - 0.001;
    pts = getPoints();
  }

  if (IsKeyPressed(KEY_J)) {
    STEP = STEP + 0.001;
    pts = getPoints();
  }

  if (IsKeyPressed(KEY_K)) {
    BOUND = BOUND - 0.1;
    pts = getPoints();
  }

  if (IsKeyPressed(KEY_L)) {
    BOUND = BOUND + 0.1;
    pts = getPoints();
  }


  if (IsKeyPressed(KEY_UP)) {
    Y_OFFSET = Y_OFFSET - 0.1;
    pts = getPoints();
  }

  if (IsKeyPressed(KEY_DOWN)) {
    Y_OFFSET = Y_OFFSET + 0.1;
    pts = getPoints();
  }

  if (IsKeyPressed(KEY_RIGHT)) {
    X_OFFSET = X_OFFSET + 0.1;
    pts = getPoints();
  }

  if (IsKeyPressed(KEY_LEFT)) {
    X_OFFSET = X_OFFSET - 0.1;
    pts = getPoints();
  }




}

void mandelbrot() {
  
  pts = getPoints();
  printf("No puntos %i", pts.size); 
  InitWindow(WINDOW_SIZE, WINDOW_SIZE, "Mandelbrot");


  while(!WindowShouldClose()) {
    BeginDrawing();
    ClearBackground(BLACK);
    HandleKeys();
    drawPoints(&pts);
    EndDrawing();
  }

  CloseWindow();
}

int main() { 
  mandelbrot();
}


