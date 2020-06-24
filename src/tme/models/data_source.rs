use serde::Deserialize;
use serde::Serialize;

use std::convert::TryInto;
use std::io::Read;
use std::str::FromStr;

use crate::tme::error::Error;
use crate::tme::models::layer::Compression;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum DataSource {
    Raw(Vec<i32>),
    Encoded(String),
}

impl DataSource {
    pub fn decode(self) -> Result<Vec<u8>, Error> {
        match &self {
            DataSource::Raw(_) => Error::InvalidDataSourceFormat(
                "data presents as raw vector, base64 encoded string expected".to_owned(),
            )
            .fail(),
            DataSource::Encoded(s) => {
                let res = decode_base64(s)?;
                Ok(res)
            }
        }
    }

    pub fn decode_and_decompress(self, compression: Compression) -> Result<Vec<i32>, Error> {
        match &self {
            DataSource::Raw(_) => Error::InvalidDataSourceFormat(
                "data presents as raw vector, base64 encoded string expected".to_owned(),
            )
            .fail(),
            DataSource::Encoded(s) => {
                let buf = decode_base64(s)?;
                let res = decompress(&buf[..], compression)?;
                Ok(res)
            }
        }
    }
}

fn decode_base64<T: AsRef<[u8]>>(s: T) -> Result<Vec<u8>, Error> {
    let result = base64::decode(s)?;
    Ok(result)
}

fn decompress(buf: &[u8], compression: Compression) -> Result<Vec<i32>, Error> {
    match compression {
        Compression::Zstd => {
            let decoder = zstd::Decoder::new(buf)?;
            decompress_with_decoder(decoder)
        }
        Compression::Zlib => {
            let decoder = flate2::read::ZlibDecoder::new(buf);
            decompress_with_decoder(decoder)
        }
        Compression::Gzip => {
            let decoder = flate2::read::GzDecoder::new(buf);
            decompress_with_decoder(decoder)
        }
    }
}

fn decompress_with_decoder<T: Read>(mut decoder: T) -> Result<Vec<i32>, Error> {
    let mut buf = Vec::new();
    let _ = decoder.read_to_end(&mut buf);

    let len = buf.len();
    let element_size = std::mem::size_of::<i32>();

    if len % element_size != 0 {
        return Error::ConvertU8SliceToPrimitive(
            "source u8 slice length not multiple of i32 size (4)".to_owned(),
        )
        .fail();
    }

    let result = buf
        .chunks(element_size)
        .into_iter()
        .map(|chunk| i32::from_le_bytes(chunk.try_into().unwrap()))
        .collect();

    Ok(result)
}

impl FromStr for DataSource {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(DataSource::Encoded(s.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tme::models::data_source::DataSource::{Encoded, Raw};
    use serde_json::json;

    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct Wrapper {
        data: DataSource,
    }

    impl Wrapper {
        pub fn new(data: DataSource) -> Self {
            Self { data }
        }
    }

    #[test]
    fn deserialize_data_source() {
        let actuals: Vec<Wrapper> = serde_json::from_value(json! {
            [
                {
                    "data": "qweasdzxcQWEASDZXC"
                },
                {
                    "data": [0, 0, 1, 0, 1]
                }
            ]
        })
        .unwrap();

        let expecteds: Vec<Wrapper> = vec![
            Wrapper::new(Encoded("qweasdzxcQWEASDZXC".to_owned())),
            Wrapper::new(Raw(vec![0, 0, 1, 0, 1])),
        ];

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn serialize_data_source() {
        let expecteds: Vec<String> = vec![
            json! {
                {
                    "data": "qweasdzxcQWEASDZXC"
                }
            },
            json! {
                {
                    "data": [0, 0, 1, 0, 1]
                }
            },
        ]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        let actuals: Vec<String> = vec![
            Wrapper::new(Encoded("qweasdzxcQWEASDZXC".to_owned())),
            Wrapper::new(Raw(vec![0, 0, 1, 0, 1])),
        ]
        .into_iter()
        .map(|v| serde_json::to_string(&v).unwrap())
        .collect();

        for (actual, expected) in actuals.into_iter().zip(expecteds) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn decompress_zlib() {
        let compression = Compression::Zlib;
        let base64 = "eJxjYmBgAAAADAAD".to_string();
        let data = DataSource::Encoded(base64);

        let actual = data.decode_and_decompress(compression).unwrap();
        let expected = vec![2];

        assert_eq!(actual, expected);
    }

    #[test]
    fn decompress_zstd() {
        let compression = Compression::Zstd;
        let base64 = "KLUv/SAEIQAAAgAAAA==".to_string();
        let data = DataSource::Encoded(base64);

        let actual = data.decode_and_decompress(compression).unwrap();
        let expected = vec![2];

        assert_eq!(actual, expected);
    }

    #[test]
    fn decompress_gzip() {
        let compression = Compression::Gzip;
        let base64 = "H4sIAAAAAAAACmNiYGAAAJcXTYsEAAAA".to_string();
        let data = DataSource::Encoded(base64);

        let actual = data.decode_and_decompress(compression).unwrap();
        let expected = vec![2];

        assert_eq!(actual, expected);
    }
}
