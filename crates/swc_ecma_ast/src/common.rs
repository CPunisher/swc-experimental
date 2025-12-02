/// Port from https://github.com/swc-project/swc/blob/main/crates/swc_ecma_ast/src/lib.rs

#[derive(Debug, Default, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum EsVersion {
    Es3,
    #[default]
    Es5,
    Es2015,
    Es2016,
    Es2017,
    Es2018,
    Es2019,
    Es2020,
    Es2021,
    Es2022,
    Es2023,
    Es2024,
    EsNext,
}

#[repr(C, align(64))]
struct Align64<T>(pub(crate) T);

const T: bool = true;
const F: bool = false;

#[inline]
pub fn is_valid_start(c: char) -> bool {
    if c.is_ascii() {
        is_valid_ascii_start(c as u8)
    } else {
        is_valid_non_ascii_start(c)
    }
}

#[inline]
pub fn is_valid_ascii_start(c: u8) -> bool {
    debug_assert!(c.is_ascii());
    // This contains `$` (36) and `_` (95)
    const ASCII_START: Align64<[bool; 128]> = Align64([
        F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F,
        F, F, F, F, F, F, T, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F,
        F, F, F, F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T,
        T, F, F, F, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T,
        T, T, T, F, F, F, F, F,
    ]);
    ASCII_START.0[c as usize]
}

pub fn is_valid_non_ascii_start(c: char) -> bool {
    debug_assert!(!c.is_ascii());
    unicode_id_start::is_id_start_unicode(c)
}

/// Returns true if `c` is a valid character for an identifier part after
/// start.
#[inline]
pub fn is_valid_continue(c: char) -> bool {
    if c.is_ascii() {
        is_valid_ascii_continue(c as u8)
    } else {
        is_valid_non_ascii_continue(c)
    }
}

#[inline]
pub fn is_valid_ascii_continue(c: u8) -> bool {
    debug_assert!(c.is_ascii());
    // This contains `$` (36)
    const ASCII_CONTINUE: Align64<[bool; 128]> = Align64([
        F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F,
        F, F, F, F, F, F, T, F, F, F, F, F, F, F, F, F, F, F, T, T, T, T, T, T, T, T, T, T, F, F,
        F, F, F, F, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T,
        T, F, F, F, F, T, F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T,
        T, T, T, F, F, F, F, F,
    ]);
    ASCII_CONTINUE.0[c as usize]
}

#[inline]
pub fn is_valid_non_ascii_continue(c: char) -> bool {
    debug_assert!(!c.is_ascii());
    unicode_id_start::is_id_continue_unicode(c)
}
