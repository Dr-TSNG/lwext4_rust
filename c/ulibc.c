#include <string.h>
#include <stdint.h>
#include <limits.h>

#include "ext4_debug.h"

// +++++++++ musl +++++++++

#define ALIGN (sizeof(size_t))
#define ONES ((size_t) - 1 / UCHAR_MAX)
#define HIGHS (ONES * (UCHAR_MAX / 2 + 1))
#define HASZERO(x) (((x) - ONES) & ~(x) & HIGHS)

__attribute__((weak))
char* __stpcpy(char* restrict d, const char* restrict s) {
#ifdef __GNUC__
    typedef size_t __attribute__((__may_alias__)) word;
    word* wd;
    const word* ws;
    if ((uintptr_t)s % ALIGN == (uintptr_t)d % ALIGN) {
        for (; (uintptr_t)s % ALIGN; s++, d++)
            if (!(*d = *s))
                return d;
        wd = (void*)d;
        ws = (const void*)s;
        for (; !HASZERO(*ws); *wd++ = *ws++);
        d = (void*)wd;
        s = (const void*)ws;
    }
#endif
    for (; (*d = *s); s++, d++);

    return d;
}

__attribute__((weak))
char* strcpy(char* restrict dest, const char* restrict src) {
    __stpcpy(dest, src);
    return dest;
}

__attribute__((weak))
int strcmp(const char* l, const char* r) {
    for (; *l == *r && *l; l++, r++);
    return *(unsigned char*)l - *(unsigned char*)r;
}

__attribute__((weak))
int strncmp(const char* _l, const char* _r, size_t n) {
    const unsigned char *l = (void*)_l, *r = (void*)_r;
    if (!n--)
        return 0;
    for (; *l && *r && n && *l == *r; l++, r++, n--);
    return *l - *r;
}

// fix me
__attribute__((weak))
FILE* const stdout = NULL;

__attribute__((weak))
int fflush(FILE* f __attribute__((unused))) {
    // printf("fflush() is not implemented !\n");
    return 0;
}

// +++++++++ uClibc +++++++++

__attribute__((weak))
void* memset(void* s, int c, size_t n) {
    register unsigned char* p = (unsigned char*)s;
    while (n) {
        *p++ = (unsigned char)c;
        --n;
    }
    return s;
}
