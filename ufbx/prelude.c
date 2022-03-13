#include <stddef.h>

__declspec(thread) const char *ufbxi_failed_assert;

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
