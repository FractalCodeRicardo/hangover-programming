#include <raylib.h>

#define SIZE 800

int main() {

  InitWindow(SIZE, SIZE, "Ray marching");

  Shader shader = LoadShader(0, "raymarch.glsl");
  int sizeLoc = GetShaderLocation(shader, "screenSize");
  float screenSize = SIZE;

  SetShaderValue(shader, sizeLoc, &screenSize, SHADER_UNIFORM_FLOAT);

  while(!WindowShouldClose()) {
    BeginDrawing();
      ClearBackground(BLACK);
        BeginShaderMode(shader);
          DrawRectangle(0, 0, SIZE, SIZE, BLACK);
        EndShaderMode();
    EndDrawing();
  }

  CloseWindow();

}
