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

atomic_int y;

void *thread_1(void *unused)
{
	atomic_load_explicit(&x.val, rlx);
	return NULL;
}

void *thread_2(void *unused)
{
        if (atomic_load_explicit(&y, rlx))
                atomic_store_explicit(&x.lower, 0x0000003, rlx);
        else
                atomic_store_explicit(&x.upper, 0x0000003, rlx);
	return NULL;
}

void *thread_3(void *unused)
{
        uint32_t expected = 0x00000001;
	atomic_compare_exchange_strong_explicit(&x.lower, &expected, 0x00000002, rlx, rlx);
	atomic_store_explicit(&x.upper, 0x0000002, rlx);
	return NULL;
}

void *thread_4(void *unused)
{
	atomic_store_explicit(&y, 1, rlx);
	return NULL;
}
