#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <stdatomic.h>
#include <genmc.h>
#include <assert.h>

#define rlx memory_order_relaxed

atomic_int x;
atomic_int y;

void *thread_1(void *unused)
{
	__VERIFIER_assume(atomic_load_explicit(&x, rlx) == 0x00000003);
	atomic_store_explicit(&y, 0x00000002, rlx);
	return NULL;
}

void *thread_2(void *unused)
{
	atomic_store_explicit(&y, 0x00000001, rlx);
	atomic_store_explicit(&x, 0x00000003, rlx);
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

	int r = y;
	assert(r == 1 || r == 2);

	return 0;
}
