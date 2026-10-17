#![cfg_attr(not(kani), no_std)]

pub use verifier;

#[no_mangle]
pub extern "C" fn entrypt() {
  test_simplify_cfg();
}

#[cfg_attr(kani, kani::proof)]
#[cfg_attr(kani, kani::unwind(3))]
fn test_simplify_cfg() {
  let v: u8 = verifier::any!();
  verifier::assume!(v < 3);   

  for i in 0..v as usize {
    return;
  }

  let sentinel: u32 = verifier::any!();
  // INV: only v == 0 gets here, so bound == 0 and sentinel >= bound holds.
  // `bound` is opaque to rustc: it is tied to `v` only through verifier::assume!,
  // an extern "C" call the front end cannot see through. Asserting against
  // `v * v` directly lets -O3 fold the assertion to `true` before SeaHorn runs.
  let bound: u32 = verifier::any!();
  verifier::assume!(bound == (v as u32) * (v as u32));
  verifier::vassert!(sentinel >= bound);
}
