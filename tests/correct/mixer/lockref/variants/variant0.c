#include "../test.c"

int idx[N];

int main() {
	thread_0();

	//pthread_t t1, t2, t3;
	//if (pthread_create(&t1, NULL, thread_1, NULL))
	//	abort();
	//if (pthread_create(&t2, NULL, thread_2, NULL))
	//	abort();
	//if (pthread_create(&t3, NULL, thread_3, NULL))
	//	abort();
	pthread_t t[N];

	for (int i = 0; i < N; i++) {
		idx[i] = i;
		if (pthread_create(&t[i], NULL, thread_n, &idx[i]))
			abort();
	}
	return 0;
}
