//#define __CONFIG_GENMC_INODE_DATA_SIZE 64

#include <stddef.h>



typedef void (*dtor_func) (void *);
struct dtor_list
{
  dtor_func func;
  void *obj;
  struct link_map *map;
  struct dtor_list *next;
};
static __thread struct dtor_list *tls_dtor_list;
static __thread void *dso_symbol_cache;
static __thread struct link_map *lm_cache;

enum
{
  PTHREAD_CREATE_JOINABLE,
#define PTHREAD_CREATE_JOINABLE	PTHREAD_CREATE_JOINABLE
  PTHREAD_CREATE_DETACHED
#define PTHREAD_CREATE_DETACHED	PTHREAD_CREATE_DETACHED
};

typedef long __VERIFIER_thread_t;
typedef struct { int __private; } __VERIFIER_attr_t;
typedef __VERIFIER_attr_t pthread_attr_t;
typedef __VERIFIER_thread_t pthread_t;


//////// stdlib

extern void __VERIFIER_free(void *) __attribute__ ((__nothrow__));

extern void *__VERIFIER_malloc(size_t) __attribute__ ((__nothrow__));

extern void *__VERIFIER_malloc_aligned(size_t, size_t) __attribute__ ((__nothrow__));

extern int __VERIFIER_atexit(void (*func)(void)) __attribute__ ((__nothrow__));


///////
extern __VERIFIER_thread_t __VERIFIER_thread_self (void) __attribute__ ((__nothrow__));


extern int __VERIFIER_thread_create (const __VERIFIER_attr_t * __attr,
				     void *(*__start_routine) (void *),
				     void *__restrict __arg) __attribute__ ((__nothrow__));
				     
extern void *__VERIFIER_thread_join (__VERIFIER_thread_t __th) __attribute__ ((__nothrow__));

				     
__attribute__ ((always_inline))
extern inline
int pthread_create(pthread_t *__restrict __newthread,
		   const pthread_attr_t *__restrict __attr,
		   void *(*__start_routine) (void *),
		   void *__restrict __arg)
{
	(*__newthread) = __VERIFIER_thread_create(__attr, __start_routine, __arg);
	return 0;
}


__attribute__ ((always_inline)) 
extern inline
int pthread_join(pthread_t __th, void **__thread_return)
{
	void *__retval = __VERIFIER_thread_join(__th);
	if (__thread_return != NULL)
		*(__thread_return) = __retval;
	return 0;
}

__attribute__ ((always_inline)) 
extern inline
int pthread_detach (pthread_t __th){
	return 0;
}

__attribute__ ((always_inline)) 
extern inline int pthread_attr_destroy (pthread_attr_t *__attr){
	return 0;
}

__attribute__ ((always_inline)) 
extern inline int pthread_attr_init (pthread_attr_t *__attr){
	return 0;
}


__attribute__ ((always_inline)) 
extern inline int pthread_attr_setstacksize (pthread_attr_t *__attr, size_t __stacksize){
	return 0;
}

extern inline __attribute__((always_inline))
void free(void *ptr)
{
	return __VERIFIER_free(ptr);
}

extern inline __attribute__((always_inline))
void *malloc(size_t size)
{
	return __VERIFIER_malloc(size);
}

extern inline __attribute__((always_inline))
void *aligned_alloc(size_t align, size_t size)
{
	return __VERIFIER_malloc_aligned(align, size);
}

extern inline __attribute__((always_inline))
int __cxa_thread_atexit_impl(void (*func) (void), void *ptr , void *ptr2)
{
	return __VERIFIER_atexit(func);
}

//extern long _ZN3std6thread7CURRENT17hc5e717b16d86dc41E = 1;
extern long _ZN3std6thread10CURRENT_ID17h6a62d35e076fe504E = 1;

extern void*   __dso_handle = (void*) &__dso_handle;

//extern long _ZN3std6thread10CURRENT_ID29_$u7b$$u7b$constant$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$3VAL17hbb1b14a078134a2dE = 0;