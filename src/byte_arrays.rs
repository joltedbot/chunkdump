use crate::errors::LocalError;
use extended::Extended;

const BYTES_IN_U8: usize = 1;
const BYTES_IN_U16: usize = 2;
const BYTES_IN_U32: usize = 4;
const BYTES_IN_U64: usize = 8;
const BYTES_IN_F64: usize = 8;
const BYTES_IN_APPLE_EXTENDED_INTEGER: usize = 10;

#[derive(Debug, Clone, PartialEq)]
pub enum Endian {
    Little,
    Big,
}

pub fn skip_over_bytes(byte_data: &mut Vec<u8>, number_of_bytes: usize) -> Result<(), LocalError> {
    check_sufficient_bytes_are_available_to_take(byte_data, number_of_bytes)?;
    let _dropping_bytes: Vec<u8> = byte_data.drain(..number_of_bytes).collect();
    Ok(())
}

pub fn take_first_byte(byte_data: &mut Vec<u8>) -> Result<u8, LocalError> {
    check_sufficient_bytes_are_available_to_take(byte_data, 1)?;
    let taken_bytes = byte_data.drain(..1).collect::<Vec<_>>();
    Ok(taken_bytes[0])
}

pub fn take_first_number_of_bytes(
    byte_data: &mut Vec<u8>,
    number_of_bytes: usize,
) -> Result<Vec<u8>, LocalError> {
    check_sufficient_bytes_are_available_to_take(byte_data, number_of_bytes)?;

    let taken_bytes: Vec<u8> = byte_data.drain(..number_of_bytes).collect();

    Ok(taken_bytes)
}

pub fn take_first_two_bytes_as_unsigned_integer(
    byte_data: &mut Vec<u8>,
    endianness: Endian,
) -> Result<u16, LocalError> {
    check_sufficient_bytes_are_available_to_take(byte_data, BYTES_IN_U16)?;

    let taken_bytes: Vec<u8> = byte_data.drain(..BYTES_IN_U16).collect();
    let mut byte_array: [u8; BYTES_IN_U16] = Default::default();
    byte_array.copy_from_slice(taken_bytes.as_slice());

    let result = match endianness {
        Endian::Little => u16::from_le_bytes(byte_array),
        Endian::Big => u16::from_be_bytes(byte_array),
    };

    Ok(result)
}

pub fn take_first_three_bytes_as_32bit_unsigned_integer(
    byte_data: &mut Vec<u8>,
    endianness: Endian,
) -> Result<u32, LocalError> {
    const NUMBER_OF_BYTES_TO_TAKE: usize = 3;
    check_sufficient_bytes_are_available_to_take(byte_data, NUMBER_OF_BYTES_TO_TAKE)?;

    let mut taken_bytes: Vec<u8> = byte_data.drain(..NUMBER_OF_BYTES_TO_TAKE).collect();

    match endianness {
        Endian::Little => {
            taken_bytes.push(0x00);
        }
        Endian::Big => {
            taken_bytes.insert(0, 0x00);
        }
    }

    let mut byte_array: [u8; BYTES_IN_U32] = Default::default();
    byte_array.copy_from_slice(taken_bytes.as_slice());

    let result = match endianness {
        Endian::Little => u32::from_le_bytes(byte_array),
        Endian::Big => u32::from_be_bytes(byte_array),
    };

    Ok(result)
}

pub fn take_first_four_bytes_as_unsigned_integer(
    byte_data: &mut Vec<u8>,
    endianness: Endian,
) -> Result<u32, LocalError> {
    check_sufficient_bytes_are_available_to_take(byte_data, BYTES_IN_U32)?;

    let taken_bytes: Vec<u8> = byte_data.drain(..BYTES_IN_U32).collect();
    let mut byte_array: [u8; BYTES_IN_U32] = Default::default();
    byte_array.copy_from_slice(taken_bytes.as_slice());

    let result = match endianness {
        Endian::Little => u32::from_le_bytes(byte_array),
        Endian::Big => u32::from_be_bytes(byte_array),
    };

    Ok(result)
}

pub fn take_first_eight_bytes_as_unsigned_integer(
    byte_data: &mut Vec<u8>,
    endianness: Endian,
) -> Result<u64, LocalError> {
    check_sufficient_bytes_are_available_to_take(byte_data, BYTES_IN_U64)?;

    let taken_bytes: Vec<u8> = byte_data.drain(..BYTES_IN_U64).collect();
    let mut byte_array: [u8; BYTES_IN_U64] = Default::default();
    byte_array.copy_from_slice(taken_bytes.as_slice());

    let result = match endianness {
        Endian::Little => u64::from_le_bytes(byte_array),
        Endian::Big => u64::from_be_bytes(byte_array),
    };

    Ok(result)
}

pub fn take_first_byte_as_signed_integer(byte_data: &mut Vec<u8>) -> Result<i8, LocalError> {
    check_sufficient_bytes_are_available_to_take(byte_data, BYTES_IN_U8)?;

    let taken_bytes: Vec<u8> = byte_data.drain(..BYTES_IN_U8).collect();
    let mut byte_array: [u8; BYTES_IN_U8] = Default::default();
    byte_array.copy_from_slice(taken_bytes.as_slice());

    Ok(i8::from_le_bytes(byte_array))
}

pub fn take_first_two_bytes_as_signed_integer(
    byte_data: &mut Vec<u8>,
    endianness: Endian,
) -> Result<i16, LocalError> {
    check_sufficient_bytes_are_available_to_take(byte_data, BYTES_IN_U16)?;

    let taken_bytes: Vec<u8> = byte_data.drain(..BYTES_IN_U16).collect();
    let mut byte_array: [u8; BYTES_IN_U16] = Default::default();
    byte_array.copy_from_slice(taken_bytes.as_slice());

    let result = match endianness {
        Endian::Little => i16::from_le_bytes(byte_array),
        Endian::Big => i16::from_be_bytes(byte_array),
    };

    Ok(result)
}

pub fn take_first_four_bytes_as_signed_integer(
    byte_data: &mut Vec<u8>,
    endianness: Endian,
) -> Result<i32, LocalError> {
    check_sufficient_bytes_are_available_to_take(byte_data, BYTES_IN_U32)?;

    let taken_bytes: Vec<u8> = byte_data.drain(..BYTES_IN_U32).collect();
    let mut byte_array: [u8; BYTES_IN_U32] = Default::default();
    byte_array.copy_from_slice(taken_bytes.as_slice());

    let result = match endianness {
        Endian::Little => i32::from_le_bytes(byte_array),
        Endian::Big => i32::from_be_bytes(byte_array),
    };

    Ok(result)
}

pub fn take_first_eight_bytes_as_signed_integer(
    byte_data: &mut Vec<u8>,
    endianness: Endian,
) -> Result<i64, LocalError> {
    check_sufficient_bytes_are_available_to_take(byte_data, BYTES_IN_U64)?;

    let taken_bytes: Vec<u8> = byte_data.drain(..BYTES_IN_U64).collect();
    let mut byte_array: [u8; BYTES_IN_U64] = Default::default();
    byte_array.copy_from_slice(taken_bytes.as_slice());

    let result = match endianness {
        Endian::Little => i64::from_le_bytes(byte_array),
        Endian::Big => i64::from_be_bytes(byte_array),
    };

    Ok(result)
}

pub fn take_first_four_bytes_as_float(
    byte_data: &mut Vec<u8>,
    endianness: Endian,
) -> Result<f32, LocalError> {
    check_sufficient_bytes_are_available_to_take(byte_data, BYTES_IN_U32)?;

    let taken_bytes: Vec<u8> = byte_data.drain(..BYTES_IN_U32).collect();
    let mut byte_array: [u8; BYTES_IN_U32] = Default::default();
    byte_array.copy_from_slice(taken_bytes.as_slice());

    match endianness {
        Endian::Little => Ok(f32::from_le_bytes(byte_array)),
        Endian::Big => Ok(f32::from_be_bytes(byte_array)),
    }
}

pub fn take_first_eight_bytes_as_float(
    byte_data: &mut Vec<u8>,
    endianness: Endian,
) -> Result<f64, LocalError> {
    check_sufficient_bytes_are_available_to_take(byte_data, BYTES_IN_F64)?;

    let taken_bytes: Vec<u8> = byte_data.drain(..BYTES_IN_F64).collect();
    let mut byte_array: [u8; BYTES_IN_F64] = Default::default();
    byte_array.copy_from_slice(taken_bytes.as_slice());

    let result = match endianness {
        Endian::Little => f64::from_le_bytes(byte_array),
        Endian::Big => f64::from_be_bytes(byte_array),
    };

    Ok(result)
}

pub fn take_first_ten_bytes_as_an_apple_extended_integer(
    byte_data: &mut Vec<u8>,
) -> Result<Extended, LocalError> {
    check_sufficient_bytes_are_available_to_take(byte_data, BYTES_IN_APPLE_EXTENDED_INTEGER)?;

    let taken_bytes: Vec<u8> = byte_data.drain(..BYTES_IN_APPLE_EXTENDED_INTEGER).collect();
    let mut byte_array: [u8; BYTES_IN_APPLE_EXTENDED_INTEGER] = Default::default();
    byte_array.copy_from_slice(taken_bytes.as_slice());

    Ok(Extended::from_be_bytes(byte_array))
}

pub fn take_first_number_of_bytes_as_string(
    byte_data: &mut Vec<u8>,
    number_of_bytes: usize,
) -> Result<String, LocalError> {
    check_sufficient_bytes_are_available_to_take(byte_data, number_of_bytes)?;

    let taken_bytes: Vec<u8> = byte_data.drain(..number_of_bytes).collect();

    let cleaned_bytes: Vec<u8> = taken_bytes
        .into_iter()
        .filter(|byte| byte.is_ascii() && *byte != 0x00 && !byte.is_ascii_control())
        .collect();

    Ok(String::from_utf8_lossy(cleaned_bytes.as_slice()).to_string())
}

fn check_sufficient_bytes_are_available_to_take(
    byte_data: &mut [u8],
    number_of_bytes_to_take: usize,
) -> Result<(), LocalError> {
    let byte_data_length = byte_data.len();
    if byte_data_length < number_of_bytes_to_take {
        return Err(LocalError::InsufficientBytesToTake(
            number_of_bytes_to_take,
            byte_data_length,
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_remove_bytes_to_be_skipped_from_the_byte_data() {
        let mut original_bytes: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
        let correct_final_bytes: Vec<u8> = vec![0x06, 0x07];
        skip_over_bytes(&mut original_bytes, 5).unwrap();
        assert!(original_bytes.eq(&correct_final_bytes));
    }

    #[test]
    fn return_correct_byte_from_the_given_bytes() {
        let mut little_endian_test_bytes: Vec<u8> = vec![0x12, 0x34, 0x56, 0x78];
        let correct_result_byte: u8 = 18;
        let result_byte: u8 = take_first_byte(&mut little_endian_test_bytes).unwrap();

        assert_eq!(result_byte, correct_result_byte);
    }

    #[test]
    fn return_correct_integer_when_taking_one_byte_as_le_unsigned_integer() {
        let mut little_endian_test_bytes: Vec<u8> = vec![0x11, 0x01, 0x01, 0x01, 0x01];
        let result_integer: u8 = take_first_byte(&mut little_endian_test_bytes).unwrap();
        let correct_result: u8 = 17;
        assert_eq!(result_integer, correct_result);
    }

    #[test]
    fn return_correct_integer_when_taking_one_byte_as_be_unsigned_integer() {
        let mut big_endian_test_bytes: Vec<u8> = vec![0x11, 0x01, 0x01, 0x01, 0x01];
        let result_integer: u8 = take_first_byte(&mut big_endian_test_bytes).unwrap();
        let correct_result: u8 = 17;
        assert_eq!(result_integer, correct_result);
    }

    #[test]
    fn return_correct_integer_when_taking_one_byte_as_signed_integer() {
        let mut little_endian_test_bytes: Vec<u8> = vec![0x11, 0x01, 0x01, 0x01, 0x01];
        let result_integer: i8 =
            take_first_byte_as_signed_integer(&mut little_endian_test_bytes).unwrap();
        let correct_result: i8 = 17;
        assert_eq!(result_integer, correct_result);
    }

    #[test]
    fn return_correct_integer_when_taking_two_bytes_as_le_unsigned_integer() {
        let mut little_endian_test_bytes: Vec<u8> = vec![0x10, 0x01, 0x01, 0x01, 0x01];
        let result_integer: u16 =
            take_first_two_bytes_as_unsigned_integer(&mut little_endian_test_bytes, Endian::Little)
                .unwrap();
        let correct_result: u16 = 272;
        assert_eq!(result_integer, correct_result);
    }

    #[test]
    fn return_correct_integer_when_taking_two_bytes_as_be_unsigned_integer() {
        let mut big_endian_test_bytes: Vec<u8> = vec![0x10, 0x01, 0x01, 0x01, 0x01];
        let result_integer: u16 =
            take_first_two_bytes_as_unsigned_integer(&mut big_endian_test_bytes, Endian::Big)
                .unwrap();
        let correct_result: u16 = 4097;
        assert_eq!(result_integer, correct_result);
    }

    #[test]
    fn return_correct_integer_when_taking_two_bytes_as_le_signed_integer() {
        let mut little_endian_test_bytes: Vec<u8> = vec![0x10, 0x01, 0x01, 0x01, 0x01];
        let result_integer: i16 =
            take_first_two_bytes_as_signed_integer(&mut little_endian_test_bytes, Endian::Little)
                .unwrap();
        let correct_result: i16 = 272;
        assert_eq!(result_integer, correct_result);
    }

    #[test]
    fn return_correct_integer_when_taking_two_bytes_as_be_signed_integer() {
        let mut big_endian_test_bytes: Vec<u8> = vec![0xF0, 0x01, 0x01, 0x01, 0x01];
        let result_integer: i16 =
            take_first_two_bytes_as_signed_integer(&mut big_endian_test_bytes, Endian::Big)
                .unwrap();
        let correct_result: i16 = -4095;
        assert_eq!(result_integer, correct_result);
    }

    #[test]
    fn return_correct_u32_integer_when_taking_three_bytes_as_le_unsigned_integer() {
        let mut test_bytes: Vec<u8> = vec![0x12, 0x34, 0x56];
        let correct_result: u32 = 5649426;
        let result =
            take_first_three_bytes_as_32bit_unsigned_integer(&mut test_bytes, Endian::Little)
                .unwrap();
        assert_eq!(result, correct_result);
    }

    #[test]
    fn return_correct_u32_integer_when_taking_three_bytes_as_be_unsigned_integer() {
        let mut test_bytes: Vec<u8> = vec![0x12, 0x34, 0x56];
        let correct_result: u32 = 1193046;
        let result =
            take_first_three_bytes_as_32bit_unsigned_integer(&mut test_bytes, Endian::Big).unwrap();
        assert_eq!(result, correct_result);
    }

    #[test]
    fn return_correct_integer_when_taking_four_bytes_as_le_unsigned_integer() {
        let mut little_endian_test_bytes: Vec<u8> = vec![0x10, 0x01, 0x01, 0x01, 0x01];
        let result_integer: u32 = take_first_four_bytes_as_unsigned_integer(
            &mut little_endian_test_bytes,
            Endian::Little,
        )
        .unwrap();
        let correct_result: u32 = 16843024;
        assert_eq!(result_integer, correct_result);
    }

    #[test]
    fn return_correct_integer_when_taking_four_bytes_as_be_unsigned_integer() {
        let mut big_endian_test_bytes: Vec<u8> = vec![0x10, 0x01, 0x01, 0x01, 0x01];
        let result_integer: u32 =
            take_first_four_bytes_as_unsigned_integer(&mut big_endian_test_bytes, Endian::Big)
                .unwrap();
        let correct_result: u32 = 268501249;
        assert_eq!(result_integer, correct_result);
    }

    #[test]
    fn return_correct_integer_when_taking_four_bytes_as_le_signed_integer() {
        let mut little_endian_test_bytes: Vec<u8> = vec![0xDD, 0xCC, 0xBB, 0xAA, 0xFF];
        let result_integer: i32 =
            take_first_four_bytes_as_signed_integer(&mut little_endian_test_bytes, Endian::Little)
                .unwrap();
        let correct_result: i32 = -1430532899;
        assert_eq!(result_integer, correct_result);
    }

    #[test]
    fn return_correct_integer_when_taking_four_bytes_as_be_signed_integer() {
        let mut big_endian_test_bytes: Vec<u8> = vec![0xDD, 0xCC, 0xBB, 0xAA, 0xFF];
        let result_integer: i32 =
            take_first_four_bytes_as_signed_integer(&mut big_endian_test_bytes, Endian::Big)
                .unwrap();
        let correct_result: i32 = -573785174;
        assert_eq!(result_integer, correct_result);
    }

    #[test]
    fn return_correct_integer_when_taking_eight_bytes_as_le_unsigned_integer() {
        let mut little_endian_test_bytes: Vec<u8> =
            vec![0x10, 0x01, 0x01, 0x01, 0x01, 0x10, 0x01, 0x01, 0x01, 0x01];
        let result_integer: u64 = take_first_eight_bytes_as_unsigned_integer(
            &mut little_endian_test_bytes,
            Endian::Little,
        )
        .unwrap();
        let correct_result: u64 = 72356665512493328;
        assert_eq!(result_integer, correct_result);
    }

    #[test]
    fn return_correct_integer_when_taking_eight_bytes_as_be_unsigned_integer() {
        let mut little_endian_test_bytes: Vec<u8> =
            vec![0x10, 0x01, 0x01, 0x01, 0x01, 0x10, 0x01, 0x01, 0x01, 0x01];
        let result_integer: u64 =
            take_first_eight_bytes_as_unsigned_integer(&mut little_endian_test_bytes, Endian::Big)
                .unwrap();
        let correct_result: u64 = 1153204083407978753;
        assert_eq!(result_integer, correct_result);
    }

    #[test]
    fn return_correct_integer_when_taking_four_bytes_as_float() {
        let mut little_endian_test_bytes: Vec<u8> = vec![0x01, 0x01, 0x01, 0x01, 0x01];
        let result_float: f32 =
            take_first_four_bytes_as_float(&mut little_endian_test_bytes, Endian::Little).unwrap();
        let correct_result: f32 = 2.3694278e-38;
        assert_eq!(format!("{}", result_float), format!("{}", correct_result));
    }

    #[test]
    fn return_correct_string_from_the_given_bytes() {
        let mut little_endian_test_bytes: Vec<u8> = vec![87, 65, 86, 69];
        let correct_result_string: String = "WAVE".to_string();
        let number_of_bytes: usize = 4;

        let result_string: String =
            take_first_number_of_bytes_as_string(&mut little_endian_test_bytes, number_of_bytes)
                .unwrap();

        assert_eq!(result_string, correct_result_string);
    }

    #[test]
    fn return_correct_value_when_taking_ten_bytes_as_apple_extended() {
        let mut test_bytes: Vec<u8> = vec![
            0x40, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF,
        ];
        let correct_result: [u8; 10] = [0x40, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

        let result = take_first_ten_bytes_as_an_apple_extended_integer(&mut test_bytes).unwrap();

        assert_eq!(result.to_be_bytes(), correct_result);
    }

    #[test]
    fn throws_error_when_available_bytes_are_less_than_number_to_be_taken() {
        let mut little_endian_test_bytes: Vec<u8> = vec![87, 65];
        let test_bytes_length = little_endian_test_bytes.len();
        let number_of_bytes_to_take: usize = 4;

        let result = check_sufficient_bytes_are_available_to_take(
            &mut little_endian_test_bytes,
            number_of_bytes_to_take,
        );

        assert_eq!(
            result.err(),
            Some(LocalError::InsufficientBytesToTake(
                number_of_bytes_to_take,
                test_bytes_length
            ))
        );
    }
}
