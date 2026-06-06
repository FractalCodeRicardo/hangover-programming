#include <stdlib.h>
#include "raylib.h"

#define SIZE_WINDOW 800
#define SIZE 40

typedef struct {
  int x;
  int y;
  char symbol;
  Color color;
} Symbol;

typedef struct {
  Symbol symbols[SIZE * SIZE];
} Screen;


Color getColor() {
  Color color =  GREEN;

  color.a = rand() % 250;

  return color;
}

char getCharacter() {
  char characters[] = {
    ' ', '$', '#',
    ' ', '4', '6',
    ' ', ' ', ' ',
    ' ', ' ', ' ',
    ' ', '1', '2',


    ' ', '9', ')',
    ' ', ')', '=',
    ' ', '!', '@',
    ' ', ',', 'n',
    ' ', 'b', 'k'
  };

  int index = rand() % 30;
  return characters[index];
}

Screen createScreen() {
  Screen screen;

  for(int x = 0; x < SIZE; x++) {
    for(int y = 0; y < SIZE; y++) {
      int index = y * SIZE + x;
      Symbol symbol;

      symbol.x = x;
      symbol.y = y;
      symbol.color = getColor();
      symbol.symbol = getCharacter();

      screen.symbols[index] = symbol;
    }
  }

  return screen;
}

void drawSymbol(Symbol *symbol) {
  char characters[2];
  characters[0] = symbol->symbol;
  characters[1] = '\0';

  int x = symbol->x;
  int y = symbol->y;
  Color color = symbol->color;
  int space = SIZE_WINDOW / SIZE;
  DrawText(characters, x * space, y * space, 20, color );
}

void drawScreen(Screen *screen) {
  for(int i; i < SIZE * SIZE; i++) {
    Symbol symbol = screen->symbols[i];
    drawSymbol(&symbol);
  }
}

void move(Screen *screen) {
  for(int i = 0; i < SIZE * SIZE; i++) {
    Symbol *symbol = &screen->symbols[i];

    int nexty = symbol -> y + 1;

    if (nexty >= SIZE) {
      nexty = 0;
    }

    symbol -> y = nexty;

  }
}

int main() {
  Screen screen = createScreen();

  SetTargetFPS(50);

  InitWindow(SIZE_WINDOW, SIZE_WINDOW, "Matrix");

  while(!WindowShouldClose()) {
    BeginDrawing();

    ClearBackground(BLACK);

    drawScreen(&screen);
    move(&screen);

    EndDrawing();
  }

  CloseWindow();
}
