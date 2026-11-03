#include <ctype.h>
#include <dirent.h>
#include <stdio.h>
#include <string.h>
#include <sys/stat.h>

void print(int level, char *name) {
  for (int i = 1; i < level; i++)
    printf("│   ");

  printf("├── %s\n", name);
}

int is_valid(char *name) {
  if (strcmp(name, ".") == 0)
    return 0;

  if (strcmp(name, "..") == 0)
    return 0;

  return 1;
}

void loop_dir(int level, char *path) {
  DIR *dir = opendir(path);

  struct dirent *e = readdir(dir);
  char full_path[500];
  struct stat status;

  while (e != NULL) {

    if (is_valid(e->d_name) == 0) {
      e = readdir(dir);
      continue;
    }

    snprintf(full_path, sizeof(full_path), "%s/%s", path, e->d_name);

    int res = stat(full_path, &status);

    if (res == -1) {
      e = readdir(dir);
      continue;
    }

    print(level, e->d_name);

    if (S_ISDIR(status.st_mode)) {
      loop_dir(level + 1, full_path);
    }

    e = readdir(dir);
  }
}

int main() { loop_dir(1, "."); }
