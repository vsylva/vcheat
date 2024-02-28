#[inline(always)]
pub(crate) fn rs_to_cwsb<S: AsRef<str>>(rs: S) -> Vec<u16> {
    if let Some(i) = rs.as_ref().find("\0") {
        rs.as_ref()[..=i].encode_utf16().collect::<Vec<u16>>()
    } else {
        format!("{}\0", rs.as_ref())
            .encode_utf16()
            .collect::<Vec<u16>>()
    }
}
