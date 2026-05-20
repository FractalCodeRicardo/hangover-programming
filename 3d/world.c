#include "raylib.h"
#include <math.h>
#include "raymath.h"
#include <stdio.h>

typedef struct {
  Camera3D camera;
} Game;

Camera3D GetCamera() {
  Camera3D camera = {0};
  camera.position = (Vector3){0.0f, 4.0f, 4.0f};
  camera.target = (Vector3){0.0f, 4, 3};
  camera.up = (Vector3){0.0f, 1.0f, 0.0f};
  camera.fovy = 60.0f;
  camera.projection = CAMERA_PERSPECTIVE;

  return camera;
}

Game GetGame() {
  Game game;
  game.camera = GetCamera();

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

Vector3 forwardDirection(Game *game) {
  Vector3 target = game -> camera.target;
  Vector3 position = game -> camera.position;

  Vector3 direction = Vector3Subtract(target, position);

  return direction;
}

void flip(Game *game, float size) {
  float sensivity = 0.005f;
  float rads = size * sensivity;

  Vector3 position = game -> camera.position;
  Vector3 forward = forwardDirection(game);

  Vector3 rotated = Vector3RotateByAxisAngle(
      forward, 
      (Vector3){0, 1, 0},
      rads
    );

  game -> camera.target = Vector3Add(position, rotated);

}

void up(Game *game) {
  Vector3 forward = forwardDirection(game);
  forward = Vector3Normalize(forward);
  MoveVector(game, forward);
}

void down(Game *game) {
  Vector3 forward = forwardDirection(game);
  Vector3 down = Vector3Normalize(forward);
  down = Vector3Scale(down, -1);

  MoveVector(game, down);
}

void right(Game *game) {
  Vector3 forward = forwardDirection(game);
  Vector3 right = Vector3Normalize(forward);
  right = Vector3Perpendicular(forward);

  MoveVector(game, right);
}

void left(Game *game) {
  Vector3 forward = forwardDirection(game);
  Vector3 left = Vector3Normalize(forward);
  left = Vector3Perpendicular(forward);
  left = Vector3Scale(left, -1);

  MoveVector(game, left);
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

  if (IsKeyDown(KEY_D)) {
    right(game);
    return;
  }

  if (IsKeyDown(KEY_A)) {
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
