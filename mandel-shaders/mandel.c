#include "raylib.h"

const int SIZE = 800;

float BOUNDS = 2.0;
float Y_OFFSET = 0.0;
float X_OFFSET = 0.0;


void HandleKeys() {

  if (IsKeyDown(KEY_UP)) {
    Y_OFFSET = Y_OFFSET - 0.02;
  }

  if (IsKeyDown(KEY_DOWN)) {
    Y_OFFSET = Y_OFFSET + 0.02;
  }

  if (IsKeyDown(KEY_RIGHT)) {
    X_OFFSET = X_OFFSET  +0.02;
  }

  if (IsKeyDown(KEY_LEFT)) {
    X_OFFSET = X_OFFSET - 0.02;
  }


  if (IsKeyDown(KEY_K)) {
    BOUNDS = BOUNDS - 0.02;
  }

  if (IsKeyDown(KEY_L)) {
    BOUNDS = BOUNDS + 0.02;
  }
}

int main() {

  InitWindow(SIZE, SIZE, "Mandelbrot");

  Shader shader = LoadShader(0, "mandel.glsl");
  
  int boundsLoc = GetShaderLocation(shader, "bounds");
  int xoffsetLoc = GetShaderLocation(shader, "xoffset");
  int yoffset = GetShaderLocation(shader, "yoffset");
  
  SetTargetFPS(60);

  while(!WindowShouldClose()) {


    HandleKeys();
    SetShaderValue(shader, boundsLoc, &BOUNDS, SHADER_UNIFORM_FLOAT);
    SetShaderValue(shader, xoffsetLoc, &X_OFFSET, SHADER_UNIFORM_FLOAT);
    SetShaderValue(shader, yoffset, &Y_OFFSET, SHADER_UNIFORM_FLOAT);


    BeginDrawing();
    ClearBackground(BLACK);

    BeginShaderMode(shader);

    DrawRectangle(0, 0, SIZE, SIZE, PURPLE);

    EndShaderMode();
    
    EndDrawing();
  }


  CloseWindow();
}
