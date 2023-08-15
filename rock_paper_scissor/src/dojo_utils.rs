
use starknet::core::types::FieldElement;


#[inline]
pub fn felt_to_f32(val: FieldElement) -> f32 {
    val.to_string().parse().unwrap()
}

#[inline]
pub fn felt_to_u32(val: FieldElement) -> u32 {
    felt_to_f32(val) as u32
}

#[inline]
pub fn u8_to_felt(val: u8) -> FieldElement {
    FieldElement::from(val)
}


pub fn slice_string(s: String, length: usize, take_last: bool) -> String {
    if take_last {
        s.chars().rev().take(length).collect::<String>().chars().rev().collect()
    } else {
        s.chars().take(length).collect()
    }
}


