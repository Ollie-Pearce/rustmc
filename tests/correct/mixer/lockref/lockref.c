#include <stdatomic.h>
/* Linux Kernel lockref implementation */
#include "lockref.h"
#include "spinlock.h"
#include <genmc.h>

// SPDX-License-Identifier: GPL-2.0

#define CMPXCHG_LOOP(CODE, SUCCESS) do {                           \
        struct lockref old;                                       \
        old.lock_count = atomic_load(&lockref->lock_count);        \
        while (old.lock.lock == 0) {                               \
                struct lockref new = old;                         \
                CODE                                              \
                if (atomic_compare_exchange_strong(&lockref->lock_count, \
                                            (int64_t *) &old.lock_count,      \
                                            new.lock_count)) {    \
                        SUCCESS;                                  \
                }                                                 \
        }                                                         \
} while (0)

/**
 * lockref_get - Increments reference count unconditionally
 * @lockref: pointer to lockref structure
 *
 * This operation is only valid if you already hold a reference
 * to the object, so you know the count cannot be zero.
 */
void lockref_get(struct lockref *lockref)
{
	CMPXCHG_LOOP(
		new.count++;
	,
		return;
	);

	spin_lock(&lockref->lock);
	lockref->count++;
	spin_unlock(&lockref->lock);
}

/**
 * lockref_get_not_zero - Increments count unless the count is 0 or dead
 * @lockref: pointer to lockref structure
 * Return: 1 if count updated successfully or 0 if count was zero
 */
int lockref_get_not_zero(struct lockref *lockref)
{
	int retval;

	CMPXCHG_LOOP(
		new.count++;
		if (old.count <= 0)
			return 0;
	,
		return 1;
	);

	spin_lock(&lockref->lock);
	retval = 0;
	if (lockref->count > 0) {
		lockref->count++;
		retval = 1;
	}
	spin_unlock(&lockref->lock);
	return retval;
}

/**
 * lockref_put_not_zero - Decrements count unless count <= 1 before decrement
 * @lockref: pointer to lockref structure
 * Return: 1 if count updated successfully or 0 if count would become zero
 */
int lockref_put_not_zero(struct lockref *lockref)
{
	int retval;

	CMPXCHG_LOOP(
		new.count--;
		if (old.count <= 1)
			return 0;
	,
		return 1;
	);

	spin_lock(&lockref->lock);
	retval = 0;
	if (lockref->count > 1) {
		lockref->count--;
		retval = 1;
	}
	spin_unlock(&lockref->lock);
	return retval;
}

/**
 * lockref_put_return - Decrement reference count if possible
 * @lockref: pointer to lockref structure
 *
 * Decrement the reference count and return the new value.
 * If the lockref was dead or locked, return an error.
 */
int lockref_put_return(struct lockref *lockref)
{
	CMPXCHG_LOOP(
		new.count--;
		if (old.count <= 0)
			return -1;
	,
		return new.count;
	);
	return -1;
}

/**
 * lockref_put_or_lock - decrements count unless count <= 1 before decrement
 * @lockref: pointer to lockref structure
 * Return: 1 if count updated successfully or 0 if count <= 1 and lock taken
 */
int lockref_put_or_lock(struct lockref *lockref)
{
	CMPXCHG_LOOP(
		new.count--;
		if (old.count <= 1)
			break;
	,
		return 1;
	);

	spin_lock(&lockref->lock);
	if (lockref->count <= 1)
		return 0;
	lockref->count--;
	spin_unlock(&lockref->lock);
	return 1;
}

/**
 * lockref_mark_dead - mark lockref dead
 * @lockref: pointer to lockref structure
 */
void lockref_mark_dead(struct lockref *lockref)
{
	//assert_spin_locked(&lockref->lock);
	lockref->count = -128;
}

/**
 * lockref_get_not_dead - Increments count unless the ref is dead
 * @lockref: pointer to lockref structure
 * Return: 1 if count updated successfully or 0 if lockref was dead
 */
int lockref_get_not_dead(struct lockref *lockref)
{
	int retval;

	CMPXCHG_LOOP(
		new.count++;
		if (old.count < 0)
			return 0;
	,
		return 1;
	);

	spin_lock(&lockref->lock);
	retval = 0;
	if (lockref->count >= 0) {
		lockref->count++;
		retval = 1;
	}
	spin_unlock(&lockref->lock);
	return retval;
}
