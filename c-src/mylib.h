struct T {
  int x;
  // for string test
  char name[4];
  // for polymorphism test
  void *ptr;
};

void plus_one(struct T *t);

int read_int(const struct T *t);
