#[macro_export]
macro_rules! define_pg_enum {
    ($name:ident {
        $($variant:ident = $val:expr),* $(,)?
    }) => {
        impl From<$name> for i16 {
            fn from(value: $name) -> Self {
                match value {
                    $($name::$variant => $val),*
                }
            }
        }

        impl std::convert::TryFrom<i16> for $name {
            type Error = String;

            fn try_from(value: i16) -> Result<Self, Self::Error> {
                match value {
                    $($val => Ok(Self::$variant),)*
                    _ => Err(format!("Invalid value for {}", stringify!($name))),
                }
            }
        }

        impl sqlx::Type<sqlx::Postgres> for $name {
            fn type_info() -> sqlx::postgres::PgTypeInfo {
                <i16 as sqlx::Type<sqlx::Postgres>>::type_info()
            }
        }

        impl sqlx::Encode<'_, sqlx::Postgres> for $name {
            fn encode_by_ref(
                &self,
                buf: &mut sqlx::postgres::PgArgumentBuffer,
            ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
                let val: i16 = (*self).into();
                <i16 as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&val, buf)
            }
        }

        impl<'r> sqlx::Decode<'r, sqlx::Postgres> for $name {
            fn decode(value: sqlx::postgres::PgValueRef<'r>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
                let val = <i16 as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
                <$name as std::convert::TryFrom<i16>>::try_from(val).map_err(|e| e.into())
            }
        }
    };
}
