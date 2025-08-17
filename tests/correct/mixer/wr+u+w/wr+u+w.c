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
			_Atomic(uint16_t) lower;
			_Atomic(uint16_t) mid;
			_Atomic(uint16_t) upper;
		};
	};
} mixed_t;

mixed_t x;

void *thread_1(void *unused)
{
	atomic_store_explicit(&x.lower, 0x0001, rlx);
	atomic_load_explicit(&x.upper, rlx);
	return NULL;
}

void *thread_2(void *unused)
{
	uint64_t expect = 0x0;
	atomic_compare_exchange_strong_explicit(&x.val, &expect, 0x000100020001, rlx, rlx);
	return NULL;
}

void *thread_3(void *unused)
{
	atomic_store_explicit(&x.mid, 0x0001, rlx);
	return NULL;
}
