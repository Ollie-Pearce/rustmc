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

atomic_int y;

void *thread_1(void *unused)
{
        atomic_store_explicit(&x.lower, 0x0002, rlx);
	__VERIFIER_assume(atomic_load_explicit(&x.upper, rlx) == 0x0001);
	return NULL;
}

void *thread_2(void *unused)
{
        atomic_store_explicit(&x.mid, 0x0002, rlx);
	return NULL;
}

void *thread_3(void *unused)
{
        uint64_t expected = 0x0000000000010001;
	atomic_compare_exchange_strong_explicit(&x.val, &expected, 0x0000000100000000, rlx, rlx);
	return NULL;
}

void *thread_4(void *unused)
{
	atomic_store_explicit(&x.lower, 0x0001, rlx);
	return NULL;
}

void *thread_5(void *unused)
{
	atomic_store_explicit(&x.mid, 0x0001, rlx);
	return NULL;
}
