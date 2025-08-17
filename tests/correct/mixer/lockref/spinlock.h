#ifndef __SPINLOCK_H
#define __SPINLOCK_H
#include <stdatomic.h>

struct spinlock_s {
        _Atomic(int32_t) lock;
};

typedef struct spinlock_s spinlock_t;

static inline void spinlock_init(struct spinlock_s *l);
static inline void spinlock_acquire(struct spinlock_s *l);
static inline int spinlock_tryacquire(struct spinlock_s *l);
static inline void spinlock_release(struct spinlock_s *l);

#define spin_lock spinlock_acquire
#define spin_unlock spinlock_release

#endif /* __SPINLOCK_H */
