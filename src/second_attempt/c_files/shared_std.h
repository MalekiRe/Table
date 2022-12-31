#pragma once


typedef __SIZE_TYPE__ size_t;
typedef __UINTPTR_TYPE__ uintptr_t;
typedef __UINT8_TYPE__ uint8_t;

#define NULL ((void *) 0)

#define STATIC_ASSERT_EQ(a, b) _Static_assert((a) == (b), "eq")

#ifndef NDEBUG
#define ASSERT(x) do { if (!(x)) __builtin_trap(); } while (0)
#else
#define ASSERT(x) do { } while (0)
#endif
#define ASSERT_EQ(a,b) ASSERT((a) == (b))

#define bool int
#define true 1
#define false 0