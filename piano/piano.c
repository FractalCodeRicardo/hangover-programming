#include "raylib.h"
#include <stdio.h>
#include <fcntl.h>
#include <string.h>

typedef struct {
  int x;
  int y;
  int width;
  int height;
  int color;
  char note[128];
  KeyboardKey key;
} Key;

typedef struct {
  Key keys[20];
  Sound sounds[20];
  int size;
} Piano;

const int WIDTH = 50;
const int HEIGTH = 200;

void DrawPianoKey(Key key) {
  Color textColor = WHITE;
  Color keyColor = WHITE;

  if (key.color == 0)  {
    keyColor = BLACK;
  } else {
    textColor = BLACK;
  }

  if (key.color == 1) {
    DrawRectangle(key.x, key.y, key.width, key.height, keyColor);
  } else {
    DrawRectangle(key.x, key.y, key.width, key.height * 0.8, keyColor);
    DrawRectangle(key.x, key.y + key.height * 0.8, key.width, key.height * 0.2, WHITE);
  }

  DrawRectangleLines(key.x, key.y, key.width, key.height, BLACK);

  DrawText(key.note, key.x + WIDTH / 2 - 20, key.y + key.height * 0.3, 5,  textColor);
}

int GetPianoKeyPressed(Piano *p) {
  for(int i = 0; i < p->size; i++) {

    if (IsKeyPressed(p->keys[i].key)) {
      return i;
    }
  }

  return -1;
}

void DrawPiano(Piano *p) {

  for(int i = 0; i < p->size; i++) {
    Key k = p->keys[i];

    DrawPianoKey(k);
  }
}


void HandleKeyPressed(Piano *p) {
  int i = GetPianoKeyPressed(p);

  if (i != -1) {
    Key key = p -> keys[i];
    printf("Key %s pressed\n", key.note); 
    PlaySound(p -> sounds[i]);
  }

}

void UnloadAudio(Piano *p) {
  for(int i = 0; i < p->size; i++) {
    UnloadSound(p->sounds[i]);
  }
}

void LoadAudio(Piano *p) {
  for(int i = 0; i < p->size; i++) {
    char soundName[128];

    strcpy(soundName, p -> keys[i].note);
    strcat(soundName, ".wav");

    printf("Loading sound %s", soundName);
    p->sounds[i] = LoadSound(soundName);
  }
}

void InitRayLib(Piano *p) {
  InitAudioDevice();
  InitWindow(800, 400, "Piano");
  LoadAudio(p);

  while(!WindowShouldClose()) {
    HandleKeyPressed(p);

    BeginDrawing();
    ClearBackground(BLACK);

    DrawPiano(p);

    EndDrawing();
  }

  UnloadAudio(p);
  CloseAudioDevice(); 
  CloseWindow();
}

Piano CreatePiano() {
  Piano p;
  //do do# re - re# mi fa - fa# sol sol# - la la# si do
  p.keys[0] = (Key) {0, 10, WIDTH, HEIGTH, 1, "C", KEY_A};
  p.keys[1] = (Key) {0, 10, WIDTH, HEIGTH, 0, "C#", KEY_S};
  p.keys[2] = (Key) {0, 10, WIDTH, HEIGTH, 1, "D", KEY_D};

  p.keys[3] = (Key) {0, 10, WIDTH, HEIGTH, 0, "D#", KEY_F};
  p.keys[4] = (Key) {0, 10, WIDTH, HEIGTH, 1, "E", KEY_G};
  p.keys[5] = (Key) {0, 10, WIDTH, HEIGTH, 1, "F", KEY_H};

  p.size = 6;

  int x = 0;
  for(int i = 0; i < p.size; i++) {
    p.keys[i].x = x;
    x += WIDTH;
  }

  return p;
}

int main() {
  Piano p = CreatePiano();
  InitRayLib(&p);
}
