#[cfg(test)]
extern crate envelope;

use envelope::Adsr;

//TODO: Tests

#[test]
fn can_inst_with_f64() {
    let _: Adsr<f64> = Adsr::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
}
#[test]
fn can_inst_with_f32() {
    let _: Adsr<f32> = Adsr::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
}
#[test]
fn can_inst_with_i8() {
    let _: Adsr<i8> = Adsr::new(0, 0, 0, 0, 0, 0, 0);
}
#[test]
fn can_inst_with_i16() {
    let _: Adsr<i16> = Adsr::new(0, 0, 0, 0, 0, 0, 0);
}
#[test]
fn can_inst_with_i32() {
    let _: Adsr<i32> = Adsr::new(0, 0, 0, 0, 0, 0, 0);
}
#[test]
fn can_inst_with_i64() {
    let _: Adsr<i64> = Adsr::new(0, 0, 0, 0, 0, 0, 0);
}
#[test]
fn can_inst_with_isize() {
    let _: Adsr<isize> = Adsr::new(0, 0, 0, 0, 0, 0, 0);
}