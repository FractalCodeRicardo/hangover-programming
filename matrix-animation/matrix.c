#include "raylib.h"
#include <stdlib.h>

#define SIZE 500

typedef struct {
  char symbol;
  Color color;
} Symbol;

typedef struct  {
  int pos;
  Symbol symbols[SIZE];
} Line;

typedef struct {
  Line lines[SIZE];
} Screen;


char getRandomChar() {
  char characters[] = {
    ' ', '$', '2'
  };
  int index = rand() % 3;  
  return characters[index];
}

Color getColor() {
  return GREEN;
}

Line createLine() {
  Line line;
  for(int i = 0;  i < SIZE ; i++) {
    line.symbols[i].symbol = getRandomChar();
    line.symbols[i].color = getColor();
  }

  return line;
}

Screen createScreen() {
  Screen screen;

  for(int i = 0; i < SIZE; i++) {
    Line line = createLine();
    screen.lines[i] = line;
    screen.lines[i].pos = i;
  }

  return screen;
}

void drawSymbol(Symbol *symbol, int x, int y) {
  char chars[2];
  chars[0] = symbol->symbol;
  chars[1] = '\0';

  DrawText(chars, x,y, 1, symbol->color);
}

void drawLine(Line *line) {
  for(int i = 0; i < SIZE; i++) {
    Symbol symbol = line ->symbols[i];
    drawSymbol(&symbol, i, line->pos);
  }
}

void drawScreen(Screen *screen) {
  for(int i = 0; i < SIZE; i++) {
    Line line = screen-> lines[i];
    drawLine(&line);
  }
}

int main() {

  Screen screen = createScreen();
  SetTargetFPS(60);
  InitWindow(SIZE, SIZE, "Matrix Rain");

  while(!WindowShouldClose()) {
    ClearBackground(BLACK);
    BeginDrawing();
    drawScreen(&screen);
    EndDrawing();
  }

  CloseWindow();




  return 0;
}
