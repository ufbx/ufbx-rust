#include <stddef.h>

#if defined(__GNUC__) || defined(__clang__) || defined(__INTEL_COMPILER)
    #define ufbxi_rust_thread_local __thread
#elif defined(_MSC_VER)
    #define ufbxi_rust_thread_local __declspec(thread)
#elif defined(__cplusplus) && (__cplusplus >= 201103L)
    #define ufbxi_rust_thread_local thread_local
#elif defined(__STDC_VERSION__) && __STDC_VERSION__ >= 201112L
    #define ufbxi_rust_thread_local _Thread_local
#else
    #define ufbxi_rust_thread_local
#endif

ufbxi_rust_thread_local const char *ufbxi_failed_assert;

void ufbxi_rust_assert_fail(const char *message)
{
    ufbxi_failed_assert = message;
}

const char *ufbxi_rust_pop_assert()
{
    const char *prev = ufbxi_failed_assert;
    ufbxi_failed_assert = NULL;
    return prev;
}
