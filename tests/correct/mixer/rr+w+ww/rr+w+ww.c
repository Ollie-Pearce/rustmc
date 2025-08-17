#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <stdatomic.h>
#include <genmc.h>
#include <assert.h>

#define rlx memory_order_relaxed

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
	atomic_load_explicit(&x.val, rlx);
	return NULL;
}

void *thread_2(void *unused)
{
	atomic_store_explicit(&x.lower, 0x0000001, rlx);
	return NULL;
}

void *thread_3(void *unused)
{
	atomic_store_explicit(&x.lower, 0x0000002, rlx);
	atomic_store_explicit(&x.upper, 0x0000001, rlx);
	return NULL;
}
