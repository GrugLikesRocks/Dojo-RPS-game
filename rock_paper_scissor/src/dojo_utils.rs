
use starknet::core::types::FieldElement;

// used to convert to and from felt to normal numbers
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

// This is used to get a snippet of a string in this case the address, either the last bit or the first bit

// Arguments:
// - s: The original string
// - length: Number of characters to take from the original string
// - take_last: If true, takes the last characters; otherwise takes the first characters
pub fn slice_string(s: String, length: usize, take_last: bool) -> String {
    if take_last {
        s.chars().rev().take(length).collect::<String>().chars().rev().collect()
    } else {
        s.chars().take(length).collect()
    }
}


