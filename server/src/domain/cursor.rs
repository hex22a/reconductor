use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use uuid::Uuid;

#[derive(Debug)]
pub(crate) enum CursorError {
    DecodeError,
    ParsingError,
}

pub(crate) fn encode_cursor(uuid: &Uuid) -> String {
    URL_SAFE_NO_PAD.encode(uuid.as_bytes())
}

pub(crate) fn decode_cursor(cursor: &str) -> Result<Uuid, CursorError> {
    let bytes = URL_SAFE_NO_PAD
        .decode(cursor)
        .map_err(|_| CursorError::DecodeError)?;
    Uuid::from_slice(&bytes).map_err(|_| CursorError::ParsingError)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_encode_cursor() {
        // Arrange
        let expected_uuid: Uuid = Uuid::from_str("019d0634-b90c-7400-99d2-253d76dfb3ea").unwrap();
        let expected_cursor = "AZ0GNLkMdACZ0iU9dt-z6g";
        // Act
        let actual_cursor = encode_cursor(&expected_uuid);
        // Assert
        assert_eq!(actual_cursor, expected_cursor);
    }

    #[test]
    fn test_decode_cursor() {
        // Arrange
        let expected_uuid: Uuid = Uuid::from_str("019d0634-b90c-7400-99d2-253d76dfb3ea").unwrap();
        let expected_cursor = "AZ0GNLkMdACZ0iU9dt-z6g";
        // Act
        let actual_uuid = decode_cursor(expected_cursor).unwrap();
        // Assert
        assert_eq!(actual_uuid, expected_uuid);
    }
}
