use ff::PrimeField;
//BLS12-381
#[derive(PrimeField, Hash)]
#[PrimeFieldModulus = "52435875175126190479447740508185965837690552500527637822603658699938581184513"]
#[PrimeFieldGenerator = "7"]
#[PrimeFieldReprEndianness = "little"]
pub struct FieldElement([u64; 4]);
