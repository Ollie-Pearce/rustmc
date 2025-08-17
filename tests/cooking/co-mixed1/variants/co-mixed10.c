#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <stdatomic.h>
#include <genmc.h>
#include <assert.h>

#define rlx __ATOMIC_RELAXED

typedef struct mixed {
	union {
		/* clang will generate atomic accesses for val */
		_Atomic(uint64_t) val;

		struct {
			_Atomic(uint32_t) upper;
			_Atomic(uint32_t) lower;
		};
	};
} mixed_t;

mixed_t x;

void *thread_1(void *unused)
{
	x.val = 0x0000000100000002;
	return NULL;
}

void *thread_2(void *unused)
{
	int a = x.upper;
	int b = x.lower;
	assert(!(a == 2 && b == 0));
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
