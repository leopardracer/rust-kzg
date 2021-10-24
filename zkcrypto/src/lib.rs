pub type Pairing = blst::Pairing;
pub type Fp = blst::blst_fp;
pub type Fp12 = blst::blst_fp12;
pub type Fp6 = blst::blst_fp6;
//pub type Fr = blst::blst_fr;
pub type P1 = blst::blst_p1;
pub type P1Affine = blst::blst_p1_affine;
pub type P2 = blst::blst_p2;
pub type P2Affine = blst::blst_p2_affine;
pub type Scalar = blst::blst_scalar;
pub type Uniq = blst::blst_uniq;

pub type ZPoly = poly::KzgPoly;

pub mod finite;
pub mod utils;
pub mod poly;
pub mod fftsettings;
pub mod consts;
pub mod zkfr;
pub mod fft_fr;

#[macro_use]
pub mod curve {
	pub mod scalar;	
}
pub type BlsScalar = curve::scalar::Scalar;

pub trait Fr : Clone {
    // Assume that Fr can't fail on creation

    // fn default() -> Self; // -> Result<Self, String>;

    fn zero() -> Self; // -> Result<Self, String>;

    fn one() -> Self; // -> Result<Self, String>;

    fn rand() -> Self; // -> Result<Self, String>;

    fn from_u64_arr(u: &[u64; 4]) -> Self;

    fn from_u64(u: u64) -> Self;

    fn is_one(&self) -> bool;

    fn is_zero(&self) -> bool;

    fn sqr(&self) -> Self;

    fn mul(&self, b: &Self) -> Self;

    fn add(&self, b: &Self) -> Self;

    fn sub(&self, b: &Self) -> Self;

    fn eucl_inverse(&self) -> Self;

    fn negate(&self) -> Self;

    fn inverse(&self) -> Self;

    fn pow(&self, n: usize) -> Self;

    fn equals(&self, b: &Self) -> bool;

    // Other teams, aside from the c-kzg bindings team, may as well leave its body empty
    fn destroy(&mut self);
}

// pub trait FFTSettings<Coeff: ZkFr>: Clone {
    // fn new(scale: usize) -> Result<Self, String>;

    // fn get_max_width(&self) -> usize;

    // fn get_expanded_roots_of_unity_at(&self, i: usize) -> Coeff;

    // fn get_expanded_roots_of_unity(&self) -> &[Coeff];

    // fn get_reverse_roots_of_unity_at(&self, i: usize) -> Coeff;

    // fn get_reversed_roots_of_unity(&self) -> &[Coeff];

    // fn destroy(&self);
// }

pub trait Poly<Coeff: Fr>: Clone {
    fn default() -> Self;

    fn new(size: usize) -> Result<Self, String>;

    fn get_coeff_at(&self, i: usize) -> Coeff;

    fn set_coeff_at(&mut self, i: usize, x: &Coeff);

    fn get_coeffs(&self) -> &[Coeff];

    fn len(&self) -> usize;

    fn eval(&self, x: &Coeff) -> Coeff;

    fn scale(&mut self);

    fn unscale(&mut self);

    fn inverse(&mut self, new_len: usize) -> Result<Self, String>;

    fn div(&mut self, x: &Self) -> Result<Self, String>;

    // Other teams, aside from the c-kzg bindings team, may as well leave its body empty
    fn destroy(&mut self);
}