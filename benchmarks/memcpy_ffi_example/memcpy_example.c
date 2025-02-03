#include <string.h>
#include <stdlib.h>

typedef struct {
    int a;
    double b;
    /* Extra data follows the structure */
} MyStruct;

__attribute__ ((annotate("c_func"))) 
void memcpy_fn() {
    size_t extraSize = 64;                    // extra uninitialized bytes
    size_t totalSize = sizeof(MyStruct) + extraSize;
    
    // Allocate a block that holds the struct plus extra uninitialized data.
    char *src = malloc(totalSize);
    char *dst = malloc(totalSize);
    
    // Initialize the struct portion in the src block.
    MyStruct *s = (MyStruct*)src;
    s->a = 42;
    s->b = 3.14;
    // The extra bytes after the struct remain uninitialized.
    
    // Copy the entire block (both the struct and the extra uninitialized data).
    memcpy(dst, src, totalSize);
    
    // Cleanup.
    free(src);
    free(dst);
}