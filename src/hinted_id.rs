use ::xid::{self as xid_};
use pgx::*;
use serde::*;
use std::ffi::CStr;
use std::str::FromStr;

type InlineShortString = smartstring::SmartString<smartstring::Compact>;

/// standard Rust equality/comparison derives
#[derive(Eq, PartialEq, Ord, Hash, PartialOrd)]
/// Pg type
#[derive(PostgresType, Serialize, Deserialize)]
/// automatically generate =, <> SQL operator functions
#[derive(PostgresEq)]
/// automatically generate <, >, <=, >=, and a "_cmp" SQL functions
/// When "PostgresEq" is also derived, pgx also creates an "opclass" (and family)
/// so that the type can be used in indexes `USING btree`
#[derive(PostgresOrd)]
/// automatically generate a "_hash" function, and the necessary "opclass" (and family)
/// so the type can also be used in indexes `USING hash`
#[derive(PostgresHash)]
/// Goodies
#[derive(Clone)]
#[repr(C)]
#[inoutfuncs]
pub struct HintedID {
    // `xid` ordered first so xid is checked first for ordering
    xid: [u8; 12],
    prefix: InlineShortString,
}

#[pg_extern]
fn generate_hinted_id(prefix: &str) -> HintedID {
    HintedID {
        prefix: prefix.into(),
        xid: xid_::new().0,
    }
}

impl InOutFuncs for HintedID {
    fn input(input: &CStr) -> Self {
        let input_str = input.to_str().expect("valid string");
        let (prefix, xid_str) = input_str
            .split_once('_')
            .expect("hintedid expected to have an underscore _ between a prefix and an xid");
        HintedID {
            prefix: prefix.into(),
            xid: xid_::Id::from_str(&xid_str)
                .expect("hintedid had invalid XID component")
                .0,
        }
    }

    fn output(&self, buffer: &mut StringInfo) {
        buffer.push_str(&self.prefix);
        buffer.push('_');
        buffer.push_str(&xid_::Id(self.xid).to_string());
    }
}
