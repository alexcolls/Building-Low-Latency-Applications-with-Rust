/* Data Races */

int shared_data = 0;

void* thread_function(void* arg) {
  shared_data++; // Unsafe write
  return NULL;
}

int main() {
  pthread_t t1, t2;
  pthread_create(&t1, NULL, thread_function, NULL);
  pthread_create(&t2, NULL, thread_function, NULL);
  pthread_join(t1, NULL);
  pthread_join(t2, NULL);
  printf("%d\n", shared_data); // Output is unpredictable
  return 0;
}

