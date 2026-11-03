// This is my atempt to create the tree tool
// in C.
//
// This is inspired by @hayyoth-org, really cool videos!
//
// I'm a noob using C, btw
//
//
#include <dirent.h>
#include <stdio.h>
#include <string.h>
#include <sys/stat.h>

void print(int level, char *name) {
  for (int i = 1; i < level; i++) {
    printf("│      ");
  }

  printf("│──  %s\n", name);
}

int is_valid(char *name) {
  if (strcmp(name, ".") == 0)
    return 0;

  if (strcmp(name, "..") == 0)
    return 0;

  return 1;
}

void show_dir(int level, char *path) {
  struct dirent **entries;
  char full_path[500];
  struct stat status;

  int size = scandir(path, &entries, NULL, alphasort);

  if (size < 0) {
    // error
    return;
  }

  for (int i = 0; i < size; i++) {
    struct dirent *e = entries[i];

    if (is_valid(e->d_name) == 0)
      continue;

    snprintf(full_path, sizeof(full_path), "%s/%s", path, e->d_name);
    int res = stat(full_path, &status);

    if (res == -1)
      continue;

    print(level, e->d_name);
    if (S_ISDIR(status.st_mode)) {
      show_dir(level + 1, full_path);
    }
  }
}

int main() { 
  show_dir(1, "."); 
}
