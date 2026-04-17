#include <stddef.h>
#include <stdint.h>

// This implementation is borrowed from sdhooghe@google.com.
// I'm not defining OT_PLATFORM_RV32, so we're falling back on the
// non-optimized version for now.
void hardened_add_mod(const uint32_t *restrict x,
                          const uint32_t *restrict y,
                          const uint32_t *restrict n, size_t word_len,
                          uint32_t *restrict dest) {
  // Randomize the content of the output buffer before writing to it.
  //hardened_memshred(dest, word_len);

  // temp_add = x + y
  uint32_t temp_add[word_len];
  uint32_t carry = 0;
  size_t count = 0;
  for (; (count) < word_len; count = (count) + 1) {
#ifdef OT_PLATFORM_RV32
    temp_add[count] = rv32_addc(x[count], y[count], &carry);
#else
    uint32_t x_val = x[count];
    uint32_t y_val = y[count];
    uint32_t res = x_val + carry;
    uint32_t next_carry = (res < carry);
    res += y_val;
    next_carry += (res < y_val);
    temp_add[count] = res;
    carry = next_carry;
#endif
  }
  //HARDENED_CHECK_EQ(count, word_len);

  // temp_sub = temp_add - n
  uint32_t temp_sub[word_len];
  uint32_t borrow = 0;
  count = 0;
  for (; (count) < word_len; count = (count) + 1) {
#ifdef OT_PLATFORM_RV32
    temp_sub[count] = rv32_subc(temp_add[count], n[count], &borrow);
#else
    uint32_t x_val = temp_add[count];
    uint32_t y_val = n[count];
    uint32_t res = x_val - borrow;
    uint32_t next_borrow = (x_val < borrow);
    next_borrow += (res < y_val);
    res -= y_val;
    temp_sub[count] = res;
    borrow = next_borrow;
#endif
  }
  //HARDENED_CHECK_EQ(count, word_len);

  uint32_t is_ge = (carry) | (1 - (borrow));

  count = 0;
  for (; (count) < word_len; count = (count) + 1) {
#ifdef OT_PLATFORM_RV32
    dest[count] = rv32_sel(is_ge, temp_sub[count], temp_add[count]);
#else
    uint32_t mask = ~(is_ge - 1);
    // Prevent optimizations of mask
    //mask = (mask);
    dest[count] = (temp_sub[count] & (mask)) |
                  (temp_add[count] & (~mask));
#endif
  }
  //HARDENED_CHECK_EQ(count, word_len);
}
