/// Modules to (de)serialize points, array/vectors of points as Base64-encoded string with serde.
use super::{PointDeserializeCompressed, PointSerializeCompressed};
use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize<S, A: PointSerializeCompressed>(p: &A, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use serde::ser::Error;

    s.serialize_str(&p.ser_base64().map_err(S::Error::custom)?)
}

pub fn deserialize<'de, D, A: PointDeserializeCompressed>(deserializer: D) -> Result<A, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;

    let base64_str = String::deserialize(deserializer)?;
    PointDeserializeCompressed::deser_base64(&base64_str).map_err(D::Error::custom)
}

pub mod array {
    /// Serialize an array of points into an array of Base64-encoded points with serde.
    use super::*;
    use arrayvec::ArrayVec;
    use serde::de::{Deserializer, SeqAccess, Visitor};
    use serde::{ser::SerializeTuple, Serializer};

    pub fn serialize<S, A: PointSerializeCompressed, const N: usize>(
        ps: &[A; N],
        s: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::Error;

        let mut seq = s.serialize_tuple(N)?;
        for p in ps {
            seq.serialize_element(&p.ser_base64().map_err(S::Error::custom)?)?;
        }
        seq.end()
    }

    pub fn deserialize<'de, D, A, const N: usize>(d: D) -> Result<[A; N], D::Error>
    where
        D: Deserializer<'de>,
        A: PointDeserializeCompressed,
    {
        use serde::de::Error;

        struct ArrayVisitor<A> {
            marker: std::marker::PhantomData<A>,
        }

        impl<'de, A, const N: usize> Visitor<'de> for ArrayVisitor<[A; N]>
        where
            A: PointDeserializeCompressed,
        {
            type Value = [A; N];

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(&format!("an array of {} elements", N))
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<[A; N], V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut arr = ArrayVec::<A, N>::new();

                for i in 0..N {
                    let value = seq
                        .next_element::<String>()?
                        .ok_or_else(|| Error::invalid_length(i, &self))?;
                    let element = A::deser_base64(&value).map_err(Error::custom)?;
                    arr.push(element);
                }

                arr.into_inner()
                    .map_err(|_| Error::invalid_length(N, &self))
            }
        }

        d.deserialize_tuple(
            N,
            ArrayVisitor {
                marker: std::marker::PhantomData,
            },
        )
    }
}

pub mod vec {
    /// Serialize a vector of points into an array of Base64-encoded points with serde.
    use super::*;
    use serde::de::{Deserializer, SeqAccess, Visitor};
    use serde::{ser::SerializeTuple, Serializer};

    pub fn serialize<S, A: PointSerializeCompressed>(ps: &[A], s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::Error;

        let mut seq = s.serialize_tuple(ps.len())?;
        for p in ps {
            seq.serialize_element(&p.ser_base64().map_err(S::Error::custom)?)?;
        }
        seq.end()
    }

    pub fn deserialize<'de, D, A>(d: D) -> Result<Vec<A>, D::Error>
    where
        D: Deserializer<'de>,
        A: PointDeserializeCompressed,
    {
        use serde::de::Error;

        struct VecVisitor<A> {
            marker: std::marker::PhantomData<A>,
        }

        impl<'de, A> Visitor<'de> for VecVisitor<A>
        where
            A: PointDeserializeCompressed,
        {
            type Value = Vec<A>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an array of base64 encoded points")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Vec<A>, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut points = Vec::new();

                while let Some(base64_str) = seq.next_element()? {
                    let p = A::deser_base64(base64_str).map_err(Error::custom)?;
                    points.push(p)
                }

                Ok(points)
            }
        }

        d.deserialize_seq(VecVisitor {
            marker: std::marker::PhantomData,
        })
    }
}
