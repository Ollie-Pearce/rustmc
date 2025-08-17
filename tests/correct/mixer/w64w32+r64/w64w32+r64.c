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
atomic_int y;

void *thread_1(void *arg)
{
	atomic_store_explicit(&x.val, 0x0000000200000001, rlx);
	atomic_store_explicit(&x.upper, 0x00000003, rlx);
	atomic_store_explicit(&y, 1, rlx);
	return NULL;
}

void *thread_2(void *arg)
{
	__VERIFIER_assume(atomic_load_explicit(&y, rlx) == 1);
	__VERIFIER_assume(atomic_load_explicit(&x.val, rlx) == 0x0000000300000001);
	return NULL;
}
