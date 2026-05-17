gcc piano.c -o app $(pkg-config --cflags --libs raylib)
./app
