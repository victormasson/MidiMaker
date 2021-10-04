#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Dynamics {
    ppp,
    pp,
    p,
    mp,
    mf,
    f,
    ff,
    fff,
}

impl Dynamics {
    pub fn get_value(self) -> u8 {
        match self {
            Dynamics::ppp => 16,
            Dynamics::pp => 32,
            Dynamics::p => 48,
            Dynamics::mp => 64,
            Dynamics::mf => 80,
            Dynamics::f => 96,
            Dynamics::ff => 112,
            Dynamics::fff => 127,
        }
    }
}
