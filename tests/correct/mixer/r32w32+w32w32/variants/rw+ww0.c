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
	__VERIFIER_assume(atomic_load_explicit(&x.lower, rlx) == 0x00000003);
	atomic_store_explicit(&x.upper, 0x00000002, rlx);
	return NULL;
}

void *thread_2(void *unused)
{
	atomic_store_explicit(&x.upper, 0x00000001, rlx);
	atomic_store_explicit(&x.lower, 0x00000003, rlx);
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

	int u = x.upper;
	assert(u == 1 || u == 2);

	return 0;
}
