#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <stdatomic.h>
#include <genmc.h>
#include <assert.h>

#define rlx __ATOMIC_RELAXED

typedef struct mixed {
	union {
		_Atomic(uint64_t) val;

		struct {
			_Atomic(uint32_t) lower;
			_Atomic(uint32_t) upper;
		};
	};
} mixed_t;

mixed_t x;

void *thread_1(void *unused)
{
	x.upper = 0x00000001;
	return NULL;
}

void *thread_2(void *unused)
{
	x.lower = 0x00000001;
	return NULL;
}

int main()
{
	pthread_t t1, t2;

	if (pthread_create(&t1, NULL, thread_1, NULL))
		abort();
	if (pthread_create(&t2, NULL, thread_2, NULL))
		abort();

	return 0;
}
