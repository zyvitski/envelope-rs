#[cfg(test)]
extern crate envelope;

use envelope::Adsr;

//TODO: Tests

#[test]
fn can_inst_with_f64() {
    let evn: Adsr<f64> = Adsr::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
}
#[test]
fn can_inst_with_f32() {
    let evn: Adsr<f32> = Adsr::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
}
#[test]
fn can_inst_with_i8() {
    let evn: Adsr<i8> = Adsr::new(0, 0, 0, 0, 0, 0, 0);
}
#[test]
fn can_inst_with_i16() {
    let evn: Adsr<i16> = Adsr::new(0, 0, 0, 0, 0, 0, 0);
}
#[test]
fn can_inst_with_i32() {
    let evn: Adsr<i32> = Adsr::new(0, 0, 0, 0, 0, 0, 0);
}
#[test]
fn can_inst_with_i64() {
    let evn: Adsr<i64> = Adsr::new(0, 0, 0, 0, 0, 0, 0);
}
#[test]
fn can_inst_with_isize() {
    let evn: Adsr<isize> = Adsr::new(0, 0, 0, 0, 0, 0, 0);
}