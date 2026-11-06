#include <raylib.h>

#define SIZE 800

float zoom = 0.0;
void HandleKeys() {

  if (IsKeyDown(KEY_UP)) {
    zoom = zoom - 0.01;
  }

  if (IsKeyDown(KEY_DOWN)) {
    zoom = zoom + 0.01;
  }
}
int main() {

  InitWindow(SIZE, SIZE, "Mandelbulb");

  Shader shader = LoadShader(0, "mandelbulb.glsl");
  int sizeLoc = GetShaderLocation(shader, "screenSize");
  int zoomLoc = GetShaderLocation(shader, "zoom");
  float screenSize = SIZE;

  SetShaderValue(
      shader, sizeLoc, 
      &screenSize, SHADER_UNIFORM_FLOAT
  );

  while(!WindowShouldClose()) {
    
    HandleKeys();

    SetShaderValue(
        shader, zoomLoc, 
        &zoom, SHADER_UNIFORM_FLOAT
    );

    BeginDrawing();
      ClearBackground(BLACK);

      BeginShaderMode(shader);
        DrawRectangle(0, 0, SIZE, SIZE, BLACK);
      EndShaderMode();

    EndDrawing();
  }

  CloseWindow();

}
