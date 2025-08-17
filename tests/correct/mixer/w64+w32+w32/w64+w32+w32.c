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
	x.val = 0x0000000100000001;
	return NULL;
}

void *thread_2(void *unused)
{
	x.upper = 0x00000002;
	return NULL;
}

void *thread_3(void *unused)
{
	x.lower = 0x00000002;
	return NULL;
}
