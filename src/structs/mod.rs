pub mod strategies;
pub mod profits;

#[derive(Clone, Copy, Debug)]
pub enum OptionType {
    Call,
    Put,
}

#[derive(Clone, Copy, Debug)]
pub enum PositionSide {
    Long,
    Short
}
impl PositionSide {
    pub fn is_long(&self) -> bool {
        match self {
            PositionSide::Long => true,
            PositionSide::Short => false,
        }
    }

    pub fn is_short(&self) -> bool {
        match self {
            PositionSide::Long => false,
            PositionSide::Short => true,
        }
    }
}