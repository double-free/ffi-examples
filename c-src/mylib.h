typedef void (*CallBack)(int *, const void *);
struct T {
  int x;
  // for char array test
  char name[4];
  // for c string test
  const char* description;
  // for polymorphism test
  CallBack callback;
  void *callback_param;
};

void plus_one(struct T *t);

int read_int(const struct T *t);

void register_callback(struct T *t, CallBack cb, void *param);
// this shall be called by C lib, but in test we call it from Rust
void trigger_callback(struct T *t);
