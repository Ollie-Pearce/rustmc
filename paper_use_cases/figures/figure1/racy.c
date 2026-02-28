#include <stdio.h>
#include <pthread.h>


int counter = 0;

void* increment_counter(void *) {
    for(int i = 0; i < 10; i++) {
        counter++;
    }
	return NULL;
}

void racy_fn() {
    pthread_t thread;

    pthread_create(&thread, NULL, increment_counter, NULL);

	pthread_join(thread, NULL);

    printf("Final counter value: %d\n", counter);

}