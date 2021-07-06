use ::xid::{self as xid_};
use pgx::*;
use std::convert::TryInto;
use std::ffi::CStr;
use std::str::FromStr;

#[derive(Copy, Clone, PostgresType)]
#[pgvarlena_inoutfuncs]
pub struct HintedID {
    prefix: [u8; 2],
    xid: xid_::Id,
}

impl PgVarlenaInOutFuncs for HintedID {
    fn input(input: &CStr) -> PgVarlena<Self> {
        let mut result = PgVarlena::<HintedID>::new();
        let input_str = input.to_str().expect("valid string");
        // 20 + 2 + 1 (_) = 23
        assert_eq!(input_str.len(), 23, "hintedid expected to be 23 characters");

        if let [p1, p2, b'_', ..] = input_str.as_bytes() {
            result.prefix = [*p1, *p2];
            result.xid =
                xid_::Id::from_str(&input_str[3..]).expect("hintedid had invalid XID component");
            result
        } else {
            panic!("hintedid expected to have a prefix");
        }
    }

    fn output(&self, buffer: &mut StringInfo) {
        buffer.push_bytes(&self.prefix);
        buffer.push('_');
        buffer.push_str(&self.xid.to_string());
    }
}

#[pg_extern]
fn generate_hinted_id(prefix: &str) -> HintedID {
    assert_eq!(
        prefix.len(),
        2,
        "hinted_id needs a prefix of exactly two characters"
    );
    HintedID {
        prefix: prefix.as_bytes().try_into().unwrap(),
        xid: xid_::new(),
    }
}

impl<'a> IntoDatum for &'a HintedID {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        let HintedID { prefix, xid } = self;
        let collected = prefix
            .into_iter()
            .chain(&xid.0)
            .copied()
            .collect::<Vec<_>>();
        let varlena = rust_byte_slice_to_bytea(collected.as_slice());
        if varlena.is_null() {
            None
        } else {
            Some(varlena.into_pg() as pg_sys::Datum)
        }
    }

    #[inline]
    fn type_oid() -> u32 {
        pg_sys::BYTEAOID
    }
}
