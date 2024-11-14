use crate::error::DataError;

pub(crate) fn crc8_matches(data: &[u8], crc: u8) -> bool {
    compute_crc8(data) == crc
}

const INITIAL: u8 = 0xFF;
const XOR: u8 = 0x31;

/// Computes a CRC-8 according to NRSC-5
/// width=8 poly=0x31 init=0xff refin=false refout=false xorout=0x00 check=0xf7 residue=0x00 name="CRC-8/NRSC-5"
pub(crate) fn compute_crc8(data: &[u8]) -> u8 {
    let mut crc = INITIAL;
    for byte in data.iter() {
        crc ^= byte;
        for _ in 0..8 {
            if (crc & 0x80) != 0 {
                crc = (crc << 1) ^ XOR;
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}

pub(crate) fn check_deserialization(data: &[u8], expected_len: usize) -> Result<(), DataError> {
    if data.len() != expected_len {
        return Err(DataError::ReceivedBufferWrongSize);
    }
    if data
        .chunks(3)
        .any(|chunk| !crc8_matches(&chunk[..2], chunk[2]))
    {
        return Err(DataError::CrcFailed);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_data_crc_computes_properly() {
        let result = compute_crc8(&[0xBE, 0xEF]);
        assert_eq!(result, 0x92);
    }

    #[test]
    fn sample_data_2_crc_computes_properly() {
        let result = compute_crc8(&[0x43, 0xDB]);
        assert_eq!(result, 0xCB);
    }

    #[test]
    fn sample_firmware_version_crc_computes_properly() {
        let result = compute_crc8(&[0x03, 0x42]);
        assert_eq!(result, 0xF3);
    }

    #[test]
    fn zero_data_crc_computes_properly() {
        let result = compute_crc8(&[0x00]);
        assert_eq!(result, 0xAC);
    }

    #[test]
    fn deserialization_with_spec_sample_works() {
        let data = [0x03, 0x42, 0xF3];
        let result = check_deserialization(&data[..], 3);
        assert!(result.is_ok());
    }

    #[test]
    fn deserialize_errors_if_buffer_to_big() {
        let data = [0x03, 0x42, 0xF3, 0x12];
        let result = check_deserialization(&data[..], 3);
        assert_eq!(result.unwrap_err(), DataError::ReceivedBufferWrongSize)
    }

    #[test]
    fn deserialize_errors_if_buffer_to_small() {
        let data = [0x03, 0x42];
        let result = check_deserialization(&data[..], 3);
        assert_eq!(result.unwrap_err(), DataError::ReceivedBufferWrongSize)
    }

    #[test]
    fn deserialize_errors_if_crc_is_wrong() {
        let data = [0x03, 0x42, 0xFF];
        let result = check_deserialization(&data[..], 3);
        assert_eq!(result.unwrap_err(), DataError::CrcFailed)
    }
}
