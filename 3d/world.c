#include "raylib.h"
#include <math.h>
#include "raymath.h"
#include <stdio.h>

const float TARGET_DISTANCE = 4.0f;
const float SPEED = 1.0f;

typedef struct {
  Camera3D camera;
  Vector3 direction;
} Game;

void RefreshTarget(Camera3D *camera, Vector3 direction) {
  Vector3 target = Vector3Scale(direction, TARGET_DISTANCE);
  target = Vector3Add(camera ->position, target);
  camera -> target = target; 
}

Camera3D GetCamera(Vector3 direction) {
  Vector3 position =  (Vector3){0, 4, 0};

  Camera3D camera = {0};
  camera.position = position;
  camera.up = (Vector3){0.0f, 1.0f, 0.0f};
  camera.fovy = 60.0f;
  camera.projection = CAMERA_PERSPECTIVE;

  RefreshTarget(&camera, direction);

  return camera;
}

Vector3 InitialDirection() {
  Vector3 direction = (Vector3) {
    1, 0, 1
  };

  direction = Vector3Normalize(direction);
  direction = Vector3Scale(direction,  SPEED);
  return direction;
}

Game GetGame() {
  Game game;
  game.direction = InitialDirection();
  game.camera = GetCamera(game.direction);

  return game;
}

void DrawGame(Game *game) {
  DrawGrid(100, 1.0f); 
}

void MoveVector(Game *game, Vector3 direction) {
  Vector3 position = Vector3Scale(direction, SPEED);
  position = Vector3Add(game->camera.position, position);
  game -> camera.position = position;

  RefreshTarget(&game ->camera, game->direction);
}


void flip(Game *game, float size) {
  float sensivity = 0.005f;
  float rads = size * sensivity;

  Vector3 position = game -> camera.position;
  Vector3 rotated = Vector3RotateByAxisAngle(
      game -> direction, 
      (Vector3){0, 1, 0},
      rads
    );

  game -> direction = rotated;
  RefreshTarget(&game -> camera, rotated);
}

void up(Game *game) {
  MoveVector(game, game -> direction);
}

void down(Game *game) {
  Vector3 down = Vector3Normalize(game -> direction);
  down = Vector3Scale(down, -1);

  MoveVector(game, down);
}

void right(Game *game) {
  Vector3 right = Vector3Perpendicular(game -> direction);
  MoveVector(game, right);
}

void left(Game *game) {
  Vector3 left = Vector3Perpendicular(game->direction);
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

  flip(game, pos.x * -1.0f);

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
