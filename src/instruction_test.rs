use super::*;


#[test]
fn test_decode() {
    assert_eq!(Instruction::decode(0x00e0), Instruction::Clear);
    assert_eq!(Instruction::decode(0x00ee), Instruction::Return);
    assert_eq!(Instruction::decode(0x1000), Instruction::Jump(0 as Addr));
}