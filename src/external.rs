use std::io::Write;
use std::io::Result as IOResult;

// TODO(babman): using offset_of! would make this be similar to Abomonation but without selfs.
pub trait Abomonation: crate::Abomonation {
    type Source;
    fn convert(t: &Self::Source) -> &Self;
    fn convert_back(&self) -> &Self::Source;
}

/*
// TODO(babman): bring this back when we move to offset_of!, then use T: ExternalAbomonation, P: ExternalAbomonation.
impl<T: crate::Abomonation> Abomonation for T {
    type Source = T;
    fn convert(t: &Self::Source) -> &Self { t }
    fn convert_back(&self) -> &Self::Source { self }
}
*/

#[inline]
pub fn encode<T, W: Write>(typed: &T::Source, write: &mut W) -> IOResult<()>
where
    T: Abomonation,
{
    let typed = T::convert(typed);
    unsafe { crate::encode(typed, write) }
}

#[inline]
pub fn decode<'a, T>(bytes: &'a mut [u8]) -> Option<(&'a T::Source, &'a mut [u8])>
where
    T: Abomonation + 'a,
{
    let result: (&T, &mut [u8]) = unsafe { crate::decode::<T>(bytes)? };
    Some((result.0.convert_back(), result.1))
}
