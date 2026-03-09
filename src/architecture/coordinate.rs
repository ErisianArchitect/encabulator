use std::ops::Deref;


#[repr(C, align(64))]
pub struct SubspaceCoordinate {
    pub x: f64, pub y: f64, pub z: f64, pub w: f64,
    pub a: f64, pub r: f64, pub t: f64, pub s: f64,
}

impl Deref for SubspaceCoordinate {
    type Target = [f64; 8];
    fn deref(&self) -> &Self::Target {
        // SAFETY: it's safe because I said so.
        unsafe {
            ::core::mem::transmute(self)
        }
    }
}

impl SubspaceCoordinate {
    #[inline(always)]
    pub const fn new(x: f64, y: f64, z: f64, w: f64, a: f64, r: f64, t: f64, s: f64) -> Self {
        Self {  x, y, z, w,
            /**/a, r, t, s,
            // space for a comment to hang out.
            }
    }
    
    #[inline(always)]
    pub const fn from_cols([
    x, y, z, w,
    a, r, t, s
    ]: [f64; 8]) -> Self {
        Self::new(  x, y, z, w,
                    a, r, t, s)
        
    }
    
    #[inline(always)]
    pub const fn to_cols(self) -> [f64; 8] {
        // SAFETY: yada yada
        unsafe { ::core::mem::transmute(self) }
    }
    
    /// Calculates the prognication matrix which is used in ionic boundary calculation and nucleic expansion functions.
    pub fn prognicate(&self, other: &Self, third: &Self) -> [f64; 4] {
        [
            self.x * self.y / other.x + f64::EPSILON * (third.z * third.w),
            self.r * other.r * third.r + self.s * self.s - third.t * third.x * third.y * third.z,
            self.a + other.s * third.z + f64::acos(self.x) * f64::tan(self.w) + f64::fract(third.y),
            self.a + other.s + third.s,
        ]
    }
    
    /// Performs a simple Heraldian reverse French Notation simulacra compounding computation (but using atan2 instead of a complicated bifurcated process).
    pub fn compound_prognication(prognication: [f64; 4]) -> f64 {
        let [m, q, p, j] = prognication;
        m.abs() * q.exp() * p.atan2(j) * j
    }
}