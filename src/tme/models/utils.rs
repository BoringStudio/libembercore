use serde::de;
use serde::de::MapAccess;
use serde::de::SeqAccess;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;

use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

use crate::tme::Error;

pub trait EndianRead {
    type Array;
    fn from_le_bytes(bytes: Self::Array) -> Self;
    fn from_be_bytes(bytes: Self::Array) -> Self;
}

macro_rules! impl_endian_read (( $($int:ident),* ) => {
    $(
        impl EndianRead for $int {
            type Array = [u8; std::mem::size_of::<Self>()];
            fn from_le_bytes(bytes: Self::Array) -> Self { Self::from_le_bytes(bytes) }
            fn from_be_bytes(bytes: Self::Array) -> Self { Self::from_be_bytes(bytes) }
        }
    )*
});

impl_endian_read!(u8, u16, u32, u64, i8, i16, i32, i64, usize, isize);

pub fn le_bytes_to_vec<'a, T>(buf: &'a [u8]) -> Result<Vec<T>, Error>
where
    T: EndianRead,
    T::Array: TryFrom<&'a [u8]>,
    <T::Array as TryFrom<&'a [u8]>>::Error: fmt::Debug,
{
    let len = buf.len();
    let element_size = std::mem::size_of::<T>();

    if len % element_size != 0 {
        return Error::ConvertBytesToPrimitive(format!(
            "source slice length ({}) not multiple of {} size ({})",
            len,
            std::any::type_name::<T>(),
            element_size
        ))
        .fail();
    }

    let result = buf
        .chunks(element_size)
        .map(|chunk| T::from_le_bytes(chunk.try_into().unwrap()))
        .collect();

    Ok(result)
}

pub fn make_true() -> bool {
    true
}

pub fn make_false() -> bool {
    false
}

pub fn make_none_option<T>() -> Option<T> {
    None
}

#[allow(dead_code)]
pub fn deserialize_value_to_string<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let v: serde_json::Value = Deserialize::deserialize(d)?;
    Ok(v.to_string())
}

#[allow(dead_code)]
pub fn deserialize_str_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr,
    D: Deserializer<'de>,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Display,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            T::from_str(value).map_err(de::Error::custom)
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>,
        {
            // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
            // into a `Deserializer`, allowing it to be used as the input to T's
            // `Deserialize` implementation. T then deserializes itself using
            // the entries from the map visitor.
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}

#[allow(dead_code)]
pub fn deserialize_str_or_seq<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr,
    D: Deserializer<'de>,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Display,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            println!("AAAAAAAAAAAA  STR");
            T::from_str(value).map_err(de::Error::custom)
        }

        fn visit_seq<S>(self, seq: S) -> Result<T, S::Error>
        where
            S: SeqAccess<'de>,
        {
            println!("AAAAAAAAAAAA  SEQ");
            Deserialize::deserialize(de::value::SeqAccessDeserializer::new(seq))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}
