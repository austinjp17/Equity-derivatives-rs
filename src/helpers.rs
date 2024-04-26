use rust_decimal::Decimal;



pub fn gen_offset_range(center: Decimal, offset: Decimal) -> Vec<Decimal> {
  
    let mut pointer = center - offset;
    let end = center + offset;
    
    let mut range = vec![];

    while pointer <= end {
        range.push(pointer);
        pointer += Decimal::ONE
    }

    range
}

/// returns vector of nums ranging from start to end
/// 
/// static step size of 1
/// 
/// start/end inclusive
pub fn gen_dec_range(start: &Decimal, end: &Decimal) -> Vec<Decimal> {
    let mut pointer = start.clone();

    let mut range = vec![];

    while pointer <= *end {
        range.push(pointer);
        pointer += Decimal::ONE
    }
    range
}

/// Variable step range
/// start/end inclusive
pub fn gen_dec_range_w_step(start: &Decimal, end: &Decimal, step: Option<Decimal>) -> Vec<Decimal> {
    let step = step.unwrap_or(Decimal::ONE);
    let mut pointer = start.clone();

    let mut range = vec![];

    while pointer <= *end {
        range.push(pointer);
        pointer += step
    }
    range
}