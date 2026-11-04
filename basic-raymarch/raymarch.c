#include "raylib.h"
#define SIZE 800
int main() {

  InitWindow(SIZE, SIZE, "Basic Raymarch");

  Shader shader = LoadShader(0, "raymarch.glsl");

  int sizeLoc = GetShaderLocation(shader, "screenSize");
  float screenSize = SIZE;
  SetShaderValue(shader, sizeLoc, &screenSize, SHADER_UNIFORM_FLOAT);


  while(!WindowShouldClose()) {

    BeginDrawing();
      ClearBackground(BLACK);
      BeginShaderMode(shader);

        DrawRectangle(0, 0, SIZE, SIZE, PURPLE);

      EndShaderMode();
    EndDrawing();
  }


  CloseWindow();
}

