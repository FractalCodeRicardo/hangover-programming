#include "raylib.h"

int const SIZE = 800;

int main() {
  InitWindow(SIZE, SIZE, "Julia Set");

  Shader shader = LoadShader(0, "julia.glsl");

  float time = 0;
  int timeLoc = GetShaderLocation(shader, "time");
  while(!WindowShouldClose()) {
    time = GetTime();

    SetShaderValue(shader, timeLoc, &time, SHADER_UNIFORM_FLOAT);
    BeginDrawing();
      ClearBackground(BLACK);

        BeginShaderMode(shader);
          DrawRectangle(0, 0, SIZE, SIZE, BLACK);
        EndShaderMode();

    EndDrawing();

  }

  CloseWindow();
}
