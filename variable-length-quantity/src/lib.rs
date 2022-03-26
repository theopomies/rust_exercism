#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

const SIGN_BIT_MASK: u8 = 1 << 7;
const VALUE_BITS_MASK: u8 = 0xff ^ SIGN_BIT_MASK;

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().copied().flat_map(vlq_encode).collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    bytes
        .iter()
        .try_fold(
            (vec![], None),
            |(mut res, constructing_value), &current_byte| {
                let constructing_value = constructing_value.unwrap_or(0);
                if constructing_value > 0x1ff_ffff {
                    return Err(Error::Overflow);
                }
                let value = (constructing_value << 7) | (current_byte & VALUE_BITS_MASK) as u32;
                if current_byte & SIGN_BIT_MASK == 0 {
                    res.push(value);
                    return Ok((res, None));
                }
                Ok((res, Some(value)))
            },
        )
        .and_then(|(res, value)| {
            (value.is_none())
                .then(|| res)
                .ok_or(Error::IncompleteNumber)
        })
}

fn vlq_encode(mut value: u32) -> Vec<u8> {
    let mut res = vec![value as u8 & VALUE_BITS_MASK];
    value >>= 7;

    while value != 0 {
        res.push((value as u8 & VALUE_BITS_MASK) | 0x80);

        value >>= 7;
    }
    res.reverse();
    res
}
