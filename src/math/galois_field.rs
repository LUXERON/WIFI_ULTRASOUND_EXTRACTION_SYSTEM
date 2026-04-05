use crate::math::alu_2048::Signed2048;
use std::ops::Mul;

impl Mul for Signed2048 {
    type Output = Self;

    /// Long multiplication mechanism unrolled across Galois field limbs.
    /// Standard $O(N^2)$ algorithm mapped carefully across the 32x64 array.
    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = Signed2048::new();
        res.sign = self.sign == rhs.sign; // Positive if both are same sign.

        for i in 0..32 {
            if self.limbs[i] == 0 {
                continue;
            }
            let mut carry_mul = 0u64;
            for j in 0..(32 - i) {
                // Determine 128-bit internal multiplication boundary
                let m = (self.limbs[i] as u128) * (rhs.limbs[j] as u128) 
                      + (res.limbs[i + j] as u128) 
                      + (carry_mul as u128);
                
                res.limbs[i + j] = m as u64; // Keep lower 64 bits
                carry_mul = (m >> 64) as u64; // Carry upper 64 bits to next
            }
        }
        res
    }
}
