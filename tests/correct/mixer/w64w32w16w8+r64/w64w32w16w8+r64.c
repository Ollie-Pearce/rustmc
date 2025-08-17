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
			_Atomic(uint8_t) zero;
			_Atomic(uint8_t) one;
			_Atomic(uint8_t) two;
			_Atomic(uint8_t) three;
			_Atomic(uint8_t) four;
			_Atomic(uint8_t) five;
			_Atomic(uint8_t) six;
			_Atomic(uint8_t) seven;
		};

		struct {
			_Atomic(uint16_t) first;
			_Atomic(uint16_t) second;
			_Atomic(uint16_t) third;
			_Atomic(uint16_t) fourth;
		};

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
	atomic_store_explicit(&x.val, 0x0807060504030201, rlx);
	atomic_store_explicit(&x.upper, 0x000A0009, rlx);
	atomic_store_explicit(&x.third, 0x0C0B, rlx);
	atomic_store_explicit(&x.seven, 0x0D, rlx);
	atomic_store_explicit(&y, 1, rlx);
	return NULL;
}

void *thread_2(void *arg)
{
	__VERIFIER_assume(atomic_load_explicit(&y, rlx) == 1);
	__VERIFIER_assume(atomic_load_explicit(&x.val, rlx) == 0x0D0A0C0B04030201);
	return NULL;
}
