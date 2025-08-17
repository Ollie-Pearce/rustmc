#include "spinlock.c"
#include "lockref.c"
#include <pthread.h>
#include <stdlib.h>

struct lockref my_lockref;

void thread_0()
{
	atomic_store(&my_lockref.lock_count, 0);
}

void *thread_1(void *unused)
{
	lockref_get(&my_lockref);
	return NULL;
}

void *thread_2(void *unused)
{
	lockref_put_return(&my_lockref);
	return NULL;
}

void *thread_3(void *unused)
{
	if (!lockref_put_or_lock(&my_lockref))
		spin_unlock(&my_lockref.lock);
	return NULL;
}

void *thread_n(void *param)
{
	lockref_get(&my_lockref);
	lockref_put_return(&my_lockref);
	return NULL;
}
