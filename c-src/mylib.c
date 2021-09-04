#include "mylib.h"
#include <stdlib.h>

void plus_one(struct T *t) {
  if (t != NULL) {
    t->x += 1;
  }
}

int read_int(const struct T *t) {
  if (t != NULL) {
    return t->x;
  }
  return -1;
}
