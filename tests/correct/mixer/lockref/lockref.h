/* Linux Kernel lockref implementation */
/*  SPDX-License-Identifier: GPL-2.0 */
#ifndef __LINUX_LOCKREF_H
#define __LINUX_LOCKREF_H

#include <stdatomic.h>
#include <stdbool.h>
#include "spinlock.h"

struct lockref {
        union {
                _Atomic(int64_t) lock_count;
                struct {
                        spinlock_t lock;
                        _Atomic(int32_t) count;
                };
        };
};

static inline void lockref_get(struct lockref *);
static inline int lockref_put_return(struct lockref *);
static inline int lockref_get_not_zero(struct lockref *);
static inline int lockref_put_not_zero(struct lockref *);
static inline int lockref_put_or_lock(struct lockref *);

static inline void lockref_mark_dead(struct lockref *);
static inline int lockref_get_not_dead(struct lockref *);

static inline bool __lockref_is_dead(const struct lockref *l)
{
                return ((int)l->count < 0);
}

#endif /*  __LINUX_LOCKREF_H */
