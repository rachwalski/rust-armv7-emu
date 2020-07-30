use rust_armv7m_emu::RegisterSet;

const GPR0: &str = "gpr0";
const GPR1: &str = "gpr1";
const GPR2: &str = "gpr2";
const GPR3: &str = "gpr3";
const GPR4: &str = "gpr4";
const GPR5: &str = "gpr5";
const GPR6: &str = "gpr6";
const GPR7: &str = "gpr7";
const GPR8: &str = "gpr8";
const GPR9: &str = "gpr9";
const GPR10: &str = "gpr10";
const GPR11: &str = "gpr11";
const GPR12: &str = "gpr12";
const SP: &str = "sp";
const LR: &str = "lr";
const PC: &str = "pc";

pub struct CPU {
    registers: RegisterSet,
}

impl CPU {
    pub fn new() -> CPU {
    
    }

}
