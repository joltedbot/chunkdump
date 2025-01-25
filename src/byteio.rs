use crate::errors::LocalError;

pub fn take_first_eight_bytes_as_integer(byte_data: &mut Vec<u8>) -> Result<u64, LocalError> {
    const NUMBER_OF_BYTES_TO_TAKE: usize = 8;
    check_sufficient_bytes_are_available(byte_data, NUMBER_OF_BYTES_TO_TAKE)?;

    let taken_bytes: Vec<u8> = byte_data.drain(..NUMBER_OF_BYTES_TO_TAKE).collect();
    let mut byte_array: [u8; NUMBER_OF_BYTES_TO_TAKE] = Default::default();
    byte_array.copy_from_slice(taken_bytes.as_slice());

    Ok(u64::from_le_bytes(byte_array))
}

pub fn take_first_four_bytes_as_integer(byte_data: &mut Vec<u8>) -> Result<u32, LocalError> {
    const NUMBER_OF_BYTES_TO_TAKE: usize = 4;
    check_sufficient_bytes_are_available(byte_data, NUMBER_OF_BYTES_TO_TAKE)?;

    let taken_bytes: Vec<u8> = byte_data.drain(..NUMBER_OF_BYTES_TO_TAKE).collect();
    let mut byte_array: [u8; NUMBER_OF_BYTES_TO_TAKE] = Default::default();
    byte_array.copy_from_slice(taken_bytes.as_slice());

    Ok(u32::from_le_bytes(byte_array))
}

pub fn take_first_two_bytes_as_integer(byte_data: &mut Vec<u8>) -> Result<u16, LocalError> {
    const NUMBER_OF_BYTES_TO_TAKE: usize = 2;
    check_sufficient_bytes_are_available(byte_data, NUMBER_OF_BYTES_TO_TAKE)?;

    let taken_bytes: Vec<u8> = byte_data.drain(..NUMBER_OF_BYTES_TO_TAKE).collect();
    let mut byte_array: [u8; NUMBER_OF_BYTES_TO_TAKE] = Default::default();
    byte_array.copy_from_slice(taken_bytes.as_slice());

    Ok(u16::from_le_bytes(byte_array))
}

pub fn take_first_four_bytes_float(byte_data: &mut Vec<u8>) -> Result<f32, LocalError> {
    const NUMBER_OF_BYTES_TO_TAKE: usize = 4;
    check_sufficient_bytes_are_available(byte_data, NUMBER_OF_BYTES_TO_TAKE)?;

    let taken_bytes: Vec<u8> = byte_data.drain(..NUMBER_OF_BYTES_TO_TAKE).collect();
    let mut byte_array: [u8; NUMBER_OF_BYTES_TO_TAKE] = Default::default();
    byte_array.copy_from_slice(taken_bytes.as_slice());

    Ok(f32::from_le_bytes(byte_array))
}

pub fn take_first_number_of_bytes_as_string(
    byte_data: &mut Vec<u8>,
    number_of_bytes: usize,
) -> Result<String, LocalError> {
    check_sufficient_bytes_are_available(byte_data, number_of_bytes)?;

    let taken_bytes: Vec<u8> = byte_data.drain(..number_of_bytes).collect();
    let cleaned_bytes: Vec<u8> = taken_bytes
        .into_iter()
        .filter(|bytes| *bytes != 0)
        .collect();

    Ok(String::from_utf8_lossy(cleaned_bytes.as_slice()).to_string())
}

pub fn take_first_number_of_bytes(
    byte_data: &mut Vec<u8>,
    number_of_bytes: usize,
) -> Result<Vec<u8>, LocalError> {
    check_sufficient_bytes_are_available(byte_data, number_of_bytes)?;

    let taken_bytes: Vec<u8> = byte_data.drain(..number_of_bytes).collect();

    Ok(taken_bytes)
}

fn check_sufficient_bytes_are_available(
    byte_data: &[u8],
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
    fn return_correct_integer_when_taking_four_bytes_as_integer() {
        let mut little_endian_test_bytes: Vec<u8> = vec![0x10, 0x01, 0x01, 0x01, 0x01];
        let result_integer: u32 =
            take_first_four_bytes_as_integer(&mut little_endian_test_bytes).unwrap();
        let correct_result: u32 = 16843024;
        assert_eq!(result_integer, correct_result);
    }

    #[test]
    fn removes_the_first_four_bytes_from_the_vector_when_taking_four_bytes_as_integer() {
        let mut little_endian_test_bytes: Vec<u8> = vec![0x10, 0x01, 0x01, 0x01, 0x01];
        let test_bytes_length_before_function_call = little_endian_test_bytes.len();

        let _ = take_first_four_bytes_as_integer(&mut little_endian_test_bytes).unwrap();
        let test_bytes_length_after_function_call = little_endian_test_bytes.len();

        assert_eq!(
            test_bytes_length_after_function_call,
            test_bytes_length_before_function_call - 4
        );
    }

    #[test]
    fn return_correct_integer_when_taking_two_bytes_as_integer() {
        let mut little_endian_test_bytes: Vec<u8> = vec![0x10, 0x01, 0x01, 0x01, 0x01];
        let result_integer: u16 =
            take_first_two_bytes_as_integer(&mut little_endian_test_bytes).unwrap();
        let correct_result: u16 = 272;
        assert_eq!(result_integer, correct_result);
    }

    #[test]
    fn removes_the_first_two_bytes_from_the_vector_when_taking_two_bytes_as_integer() {
        let mut little_endian_test_bytes: Vec<u8> = vec![0x10, 0x01, 0x01, 0x01, 0x01];
        let test_bytes_length_before_function_call = little_endian_test_bytes.len();

        let _ = take_first_two_bytes_as_integer(&mut little_endian_test_bytes).unwrap();
        let test_bytes_length_after_function_call = little_endian_test_bytes.len();

        assert_eq!(
            test_bytes_length_after_function_call,
            test_bytes_length_before_function_call - 2
        );
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
    fn removes_the_given_number_of_bytes_from_the_vector_when_taking_number_of_bytes_as_string() {
        let mut little_endian_test_bytes: Vec<u8> = vec![87, 65, 86, 69, 1, 2, 3];
        let test_bytes_length_before_function_call = little_endian_test_bytes.len();
        let number_of_bytes: usize = 4;

        let _: String =
            take_first_number_of_bytes_as_string(&mut little_endian_test_bytes, number_of_bytes)
                .unwrap();

        let test_bytes_length_after_function_call = little_endian_test_bytes.len();

        assert_eq!(
            test_bytes_length_after_function_call,
            test_bytes_length_before_function_call - number_of_bytes,
        );
    }

    #[test]
    fn throws_error_when_available_bytes_are_less_than_number_to_be_taken() {
        let little_endian_test_bytes: Vec<u8> = vec![87, 65];
        let test_bytes_length = little_endian_test_bytes.len();
        let number_of_bytes_to_take: usize = 4;

        let result = check_sufficient_bytes_are_available(
            &little_endian_test_bytes,
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
