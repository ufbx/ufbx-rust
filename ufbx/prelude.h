#ifndef UFBX_RUST_PRELUDE_H
#define UFBX_RUST_PRELUDE_H

void ufbxi_rust_assert_fail(const char *message);
const char *ufbxi_rust_pop_assert();

#define ufbxi_rust_assert_imp(cond, file, line) \
    do { if (!(cond)) ufbxi_rust_assert_fail(file ":" #line ": ufbx_assert(" #cond ")"); } while (0)

#define ufbxi_rust_assert(cond, file, line) \
    ufbxi_rust_assert_imp(cond, file, line)

#define ufbx_assert(cond) ufbxi_rust_assert(cond, __FILE__, __LINE__)

#endif
