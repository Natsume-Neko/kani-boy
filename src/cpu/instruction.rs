pub enum Instruction {
    Add(ArithmeticTarget),
    Adc(ArithmeticTarget),
    Sub(ArithmeticTarget),
    Sbc(ArithmeticTarget),
    And(ArithmeticTarget),
    Or(ArithmeticTarget),
    Xor(ArithmeticTarget),
    Cp(ArithmeticTarget),
    Inc(ArithmeticTarget),
    Dec(ArithmeticTarget),
    AddHL(ArithmeticTarget16Bit),
    Inc16Bit(ArithmeticTarget16Bit),
    Dec16Bit(ArithmeticTarget16Bit),
    Swap(ArithmeticTarget),
    Cpl,
    Ccf,
    Scf,
    Rlca,
    Rla,
    Rrca,
    Rra,
    Rlc(ArithmeticTarget),
    Rl(ArithmeticTarget),
    Rrc(ArithmeticTarget),
    Rr(ArithmeticTarget),
    Sla(ArithmeticTarget),
    Sra(ArithmeticTarget),
    Srl(ArithmeticTarget),
    Bit((u8, ArithmeticTarget)),
    Set((u8, ArithmeticTarget)),
    Res((u8, ArithmeticTarget)),
}

pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    // [TODO]: (HL)
}

pub enum ArithmeticTarget16Bit {
    BC,
    DE,
    HL,
    // [TODO]: SP
}
