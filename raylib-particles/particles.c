#include <stdlib.h>
#include <time.h>
#include "raylib.h"

#define WINDOW_SIZE 700
#define SIZE 2000

typedef struct {
  int x;
  int y;
  int vx;
  int vy;
  Color color;
} Point;

typedef struct {
  Point points[SIZE]
} Particles;


Color RandomColor() {
  Color colors[] = {
    GREEN,
    YELLOW,
    BLUE,
    PURPLE,
    ORANGE,
    RED
  };

  return colors[(int)(rand() % 6)];
}

int RandomPos() {
    int x = rand() % WINDOW_SIZE;
    return x;
}

int RandomVel() {
  int sign = rand() % 2  == 1 ? -1 : 1;
  int number = rand() % 3 + 1;
  return sign * number;
}

Particles CreateParticles() {
  Particles particles;

  for(int i = 0; i < SIZE; i ++) {
    int y = RandomPos();
    int x = RandomPos();
    Color color = RandomColor();
    int vx = RandomVel();
    int vy = RandomVel();

    particles.points[i] = (Point) {x, y, vx, vy, color};

  }

  return particles;
}

void DrawParticles(Particles *particles) {

  for(int i = 0; i < SIZE; i ++) {
    Point p = particles -> points[i];
    DrawCircle(p.x, p.y, 3, p.color);
  }
}

void MoveParticles(Particles * particles) {

  for(int i = 0; i < SIZE; i ++) {
    Point p = particles -> points[i];

    p.x += p.vx;
    p.y += p.vy;

    if (p.x <= 0 || p.x >= WINDOW_SIZE) {
      p.vx *= -1;
      p.x += p.vx;
    }


    if (p.y <= 0 || p.y >= WINDOW_SIZE) {
      p.vy *= -1;
      p.y += p.vy;
    }

    particles -> points[i] = p;
  }
}

int main() {

  srand(time(NULL));
  SetTargetFPS(60);

  Particles particles = CreateParticles();

  InitWindow(WINDOW_SIZE, WINDOW_SIZE, "Particles");

  while(!WindowShouldClose()) {
    BeginDrawing();
    ClearBackground(BLACK);
    DrawParticles(&particles);
    MoveParticles(&particles);
    EndDrawing();
  }

  CloseWindow();
  return 0;
}
