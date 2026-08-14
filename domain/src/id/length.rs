use arrayvec::ArrayString;

pub(crate) fn id_from<const CAP: usize, T, Str>(
    id: Str,
    function: impl Fn(ArrayString<CAP>) -> T,
) -> Result<T, IdLengthError>
where
    Str: AsRef<str>,
{
    let id = id.as_ref();

    ArrayString::from(id)
        .map(function)
        .map_err(|_| IdLengthError {
            length: id.len(),
            max_length: CAP,
        })
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub struct IdLengthError {
    pub length: usize,
    pub max_length: usize,
}
