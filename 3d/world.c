#include "raylib.h"
#include <math.h>
#include "raymath.h"

typedef struct {
  Camera3D camera;
  float cameraAngle;
} Game;

Camera3D GetCamera() {
  Camera3D camera = {0};
  camera.position = (Vector3){0.0f, 4.0f, 4.0f};
  camera.target = (Vector3){0.0f, 4, 3};
  camera.up = (Vector3){0.0f, 1.0f, 0.0f};
  camera.fovy = 10.0f;
  camera.projection = CAMERA_PERSPECTIVE;

  return camera;
}

Game GetGame() {
  Game game;
  game.camera = GetCamera();
  game.cameraAngle = 0;

  return game;
}

void DrawGame(Game *game) {
  DrawGrid(100, 1.0f); 
}

void MoveVector(Game *game, Vector3 v) {
  Vector3 pos = game->camera.position;
  Vector3 target = game->camera.target;

  game -> camera.position = (Vector3) {
    pos.x + v.x,
    pos.y + v.y,
    pos.z + v.z
  };


  game -> camera.target = (Vector3) {
    target.x + v.x,
    target.y + v.y,
    target.z + v.z
  };
}

void Flip(Game *game, float size) {

  float sensivity = 0.5f;
  float angle = game -> cameraAngle;
  float newAngle = angle + (size * sensivity);
  float rads = newAngle * DEG2RAD;

  Vector3 target = game -> camera.target;
  Vector3 position = game -> camera.position;

  Vector3 direction = Vector3Subtract(target, position);

  float newx = direction.x * cosf(rads) + direction.z * sinf(rads);
  float newz = -1 * direction.x * sinf(0) + direction.z * cosf(rads);
  Vector3 rotated = (Vector3) {
    newx,
    direction.y,
    newz
  };

  game -> camera.target = Vector3Add(position, rotated);
  game -> cameraAngle = newAngle;

}

void up(Game *game) {
  MoveVector(game, (Vector3){0,0,-0.1});
}

void down(Game *game) {
  MoveVector(game, (Vector3){0,0,0.1});
}

void right(Game *game) {
  MoveVector(game, (Vector3){0.1,0,0});
}

void left(Game *game) {
  MoveVector(game, (Vector3){-0.1,0,0});
}

void HandleKeys(Game *game) {
  if (IsKeyDown(KEY_W)) {
    up(game);
    return;
  }

  if (IsKeyDown(KEY_S)) {
    down(game);
    return;
  }

  if (IsKeyDown(KEY_A)) {
    right(game);
    return;
  }

  if (IsKeyDown(KEY_B)) {
    left(game);
    return;
  }
}

void HandleMouse(Game *game) {
  Vector2 pos = GetMouseDelta();

  if (pos.x == 0) {
    return;
  }

  Flip(game, pos.x);

}

int main() {

  Game game = GetGame();

  InitWindow(800, 800, "World");

  SetTargetFPS(60);

  while (!WindowShouldClose()) {
    BeginDrawing();
    ClearBackground(RAYWHITE);
    BeginMode3D(game.camera);

    DrawGame(&game);
    HandleKeys(&game);
    HandleMouse(&game);

    EndMode3D();
    EndDrawing();
  }

  CloseWindow();

  return 0;
}
