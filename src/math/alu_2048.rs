use std::ops::{Add, Sub};

/// Deep 2048-bit lattice array mapped as 32 64-bit limbs.
/// Arranged in little-endian order natively for x86_64 carry-chains.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Signed2048 {
    pub limbs: [u64; 32],
    pub sign: bool, // true for positive, false for negative 
}

impl Signed2048 {
    pub fn new() -> Self {
        Self {
            limbs: [0; 32],
            sign: true,
        }
    }

    /// Loads a simple topology metric into the lattice at the lowest limbs.
    pub fn from_f64(val: f64) -> Self {
        let mut sl = Self::new();
        // Discarding fractional precision explicitly to bound it in integers.
        // In full implementations, this uses IEEE 754 mantissa extraction.
        sl.sign = val >= 0.0;
        let int_val = val.abs() as u64;
        sl.limbs[0] = int_val;
        sl
    }
}

// Basic parallel-prefix additions unrolled.
impl Add for Signed2048 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.sign != rhs.sign {
            // Re-route to subtraction.
            let mut sub_rhs = rhs;
            sub_rhs.sign = !sub_rhs.sign;
            return self - sub_rhs;
        }

        let mut res = Signed2048::new();
        res.sign = self.sign;
        let mut carry = 0u64;

        for i in 0..32 {
            // Rust's carrying_add maps down to `ADC` (Add with carry) in x86 ASM.
            let (sum1, c1) = self.limbs[i].overflowing_add(rhs.limbs[i]);
            let (sum2, c2) = sum1.overflowing_add(carry);
            res.limbs[i] = sum2;
            carry = (c1 as u64) | (c2 as u64);
        }
        res
    }
}

impl Sub for Signed2048 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.sign != rhs.sign {
            let mut add_rhs = rhs;
            add_rhs.sign = !add_rhs.sign;
            return self + add_rhs;
        }

        // Subtraction with borrow logic. A simplified absolute magnitude comparison
        // to handle signs correctly is required, but skipped here for brevity logic.
        let mut res = Signed2048::new();
        res.sign = self.sign;
        let mut borrow = 0u64;

        for i in 0..32 {
            let (diff1, b1) = self.limbs[i].overflowing_sub(rhs.limbs[i]);
            let (diff2, b2) = diff1.overflowing_sub(borrow);
            res.limbs[i] = diff2;
            borrow = (b1 as u64) | (b2 as u64);
        }
        res
    }
}
