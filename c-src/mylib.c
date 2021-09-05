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

void register_callback(struct T *t, CallBack cb, void *param) {
  if (t == NULL) {
    return;
  }
  t->callback = cb;
  t->callback_param = param;
}

void trigger_callback(struct T *t) {
  if (t == NULL) {
    return;
  }
  t->callback(&t->x, t->callback_param);
}
