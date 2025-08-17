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

int a;
int b;

void *thread_1(void *unused)
{
	atomic_store_explicit(&x.val, 0x0000000200000001, rlx);
	return NULL;
}

void *thread_2(void *unused)
{
	a = atomic_load_explicit(&x.lower, rlx);
	b = atomic_load_explicit(&x.upper, rlx);;
	return NULL;
}

int main()
{
	pthread_t t1, t2;

	if (pthread_create(&t1, NULL, thread_1, NULL))
		abort();
	if (pthread_create(&t2, NULL, thread_2, NULL))
		abort();

	if (pthread_join(t1, NULL))
		abort();
	if (pthread_join(t2, NULL))
		abort();

	assert(!(a == 1 && b == 0));

	return 0;
}
