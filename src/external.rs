use std::io::{Write, Result as IOResult};
use std::marker::PhantomData;
use Abomonation;
use external::sealed::IdentityDescription;

extern crate memoffset;
pub use self::memoffset::offset_of;

extern crate derive;
pub use self::derive::external_abomoniate;

// Describes a struct field: both its type and its offset in memory.
pub struct StructDescriptor<T> {
    t: PhantomData<T>,
    offset: usize,
}
impl<T> StructDescriptor<T> {
    pub const fn new(offset: usize) -> Self {
        StructDescriptor { t: PhantomData, offset }
    }
    #[inline]
    pub const fn offset(&self) -> usize {
        self.offset
    }
}

// Private Trait.
mod sealed {
    use std::io::{Write, Result as IOResult};
    use std::marker::PhantomData;
    use Abomonation;
    use external::{ExternalAbomonation, StructDescriptor};

    extern crate paste;
    use self::paste::paste;

    pub trait StructDescription {
        unsafe fn entomb<W: Write>(&self, source: &[u8], write: &mut W) -> IOResult<()>;
        unsafe fn exhume<'a, 'b>(&self, source: &'a mut [u8], bytes: &'b mut [u8]) -> Option<&'b mut [u8]>;
        unsafe fn extent(&self, source: &[u8]) -> usize;
    }

    macro_rules! tuple_descriptor {
        ( $($name:ident)+) => {
            paste! {
                impl<$($name: ExternalAbomonation,)*> StructDescription for ($(StructDescriptor<$name>,)*) {
                    #[allow(non_snake_case)]
                    #[inline(always)]
                    unsafe fn entomb<WRITE: Write>(&self, source: &[u8], write: &mut WRITE) -> IOResult<()> {
                        let ($([<$name _VAR>],)*) = self;
                        $($name::DESCRIPTION.entomb(source.split_at([<$name _VAR>].offset()).1, write)?;)*
                        Ok(())
                    }

                    #[allow(non_snake_case)]
                    #[inline(always)]
                    unsafe fn exhume<'a,'b>(&self, mut source: &'a mut [u8], mut bytes: &'b mut [u8]) -> Option<&'b mut [u8]> {
                        let ($([<$name _VAR>],)*) = self;
                        let len = source.len();
                        $(
                        let temp = bytes;
                        let (source1, source2) = source.split_at_mut([<$name _VAR>].offset());
                        bytes = $name::DESCRIPTION.exhume(source2, temp)?;
                        source = std::slice::from_raw_parts_mut(source1.as_mut_ptr(), source2.len());
                        )*
                        assert_eq!(source.len(), len);
                        Some(bytes)
                    }

                    #[allow(non_snake_case)]
                    #[inline(always)]
                    unsafe fn extent(&self, source: &[u8]) -> usize {
                        let ($([<$name _VAR>],)*) = self;
                        let mut size = 0;
                        $( size += $name::DESCRIPTION.extent(source.split_at([<$name _VAR>].offset()).1); )*
                        size
                    }
                }
            }
        }
    }

    tuple_descriptor!(A);
    tuple_descriptor!(A B);
    tuple_descriptor!(A B C);
    tuple_descriptor!(A B C D);
    tuple_descriptor!(A B C D E);
    tuple_descriptor!(A B C D E F);
    tuple_descriptor!(A B C D E F G);
    tuple_descriptor!(A B C D E F G H);
    tuple_descriptor!(A B C D E F G H I);
    tuple_descriptor!(A B C D E F G H I J);
    tuple_descriptor!(A B C D E F G H I J K);
    tuple_descriptor!(A B C D E F G H I J K L);
    tuple_descriptor!(A B C D E F G H I J K L M);
    tuple_descriptor!(A B C D E F G H I J K L M N);
    tuple_descriptor!(A B C D E F G H I J K L M N O);
    tuple_descriptor!(A B C D E F G H I J K L M N O P);
    tuple_descriptor!(A B C D E F G H I J K L M N O P Q);
    tuple_descriptor!(A B C D E F G H I J K L M N O P Q R);
    tuple_descriptor!(A B C D E F G H I J K L M N O P Q R S);
    tuple_descriptor!(A B C D E F G H I J K L M N O P Q R S T);
    tuple_descriptor!(A B C D E F G H I J K L M N O P Q R S T U);
    tuple_descriptor!(A B C D E F G H I J K L M N O P Q R S T U V);
    tuple_descriptor!(A B C D E F G H I J K L M N O P Q R S T U V W);
    tuple_descriptor!(A B C D E F G H I J K L M N O P Q R S T U V W X);
    tuple_descriptor!(A B C D E F G H I J K L M N O P Q R S T U V W X Y);
    tuple_descriptor!(A B C D E F G H I J K L M N O P Q R S T U V W X Y Z);
    tuple_descriptor!(A B C D E F G H I J K L M N O P Q R S T U V W X Y Z AA);
    tuple_descriptor!(A B C D E F G H I J K L M N O P Q R S T U V W X Y Z AA AB);
    tuple_descriptor!(A B C D E F G H I J K L M N O P Q R S T U V W X Y Z AA AB AC);
    tuple_descriptor!(A B C D E F G H I J K L M N O P Q R S T U V W X Y Z AA AB AC AD);
    tuple_descriptor!(A B C D E F G H I J K L M N O P Q R S T U V W X Y Z AA AB AC AD AE);
    tuple_descriptor!(A B C D E F G H I J K L M N O P Q R S T U V W X Y Z AA AB AC AD AE AF);

    // For types that are abomonitable directly.
    pub struct IdentityDescription<T> {
        t: PhantomData<T>,
    }
    impl<T> IdentityDescription<T> {
        pub const fn new() -> Self {
            Self { t: PhantomData }
        }
    }
    impl<T: Abomonation> StructDescription for IdentityDescription<T> {
        unsafe fn entomb<W: Write>(&self, source: &[u8], write: &mut W) -> IOResult<()> {
            let t: &T = std::mem::transmute(source.get_unchecked(0));
            t.entomb(write)
        }
        unsafe fn exhume<'a, 'b>(&self, source: &'a mut [u8], bytes: &'b mut [u8]) -> Option<&'b mut [u8]> {
            let t: &mut T = std::mem::transmute(source.get_unchecked_mut(0));
            t.exhume(bytes)
        }
        unsafe fn extent(&self, source: &[u8]) -> usize {
            let t: &T = std::mem::transmute(source.get_unchecked(0));
            t.extent()
        }
    }
}

// Allows to abomonate instances of source.
// Auto-implemented by macro for generated structs.
pub trait ExternalAbomonation {
    // External type we want to abomonate.
    type Source;
    // A description of the external type.
    // This is a private trait, it is only implemented by tuples that contain StructDescriptor<T> of different Ts.
    type Description: sealed::StructDescription;
    // An instance of description that stores the offsets.
    const DESCRIPTION: Self::Description;
}

// Every Abomonatable type is also externally abomonatable.
impl<T: Abomonation> ExternalAbomonation for T {
    type Source = T;
    type Description = IdentityDescription<T>;
    const DESCRIPTION: Self::Description = Self::Description::new();
}

// Encoding instances of T::Source.
#[inline]
pub unsafe fn external_encode<T: ExternalAbomonation, W: Write>(typed: &T::Source, write: &mut W) -> IOResult<()> {
    use external::sealed::StructDescription;
    let slice = std::slice::from_raw_parts(std::mem::transmute(typed), std::mem::size_of::<T::Source>());
    write.write_all(slice)?;
    T::DESCRIPTION.entomb(slice, write)?;
    Ok(())
}

// Decoding instances of T::Source.
#[inline]
pub unsafe fn extenral_decode<T: ExternalAbomonation>(bytes: &mut [u8]) -> Option<(&T::Source, &mut [u8])> {
    use external::sealed::StructDescription;
    if bytes.len() < std::mem::size_of::<T::Source>() { None }
    else {
        let (split1, split2) = bytes.split_at_mut(std::mem::size_of::<T::Source>());
        if let Some(remaining) = T::DESCRIPTION.exhume(split1, split2) {
            let result: &mut T::Source = std::mem::transmute(split1.get_unchecked_mut(0));
            Some((result, remaining))
        }
        else {
            None
        }
    }
}

// Measure size of buffer for instances of T::Source.
#[inline]
pub unsafe fn external_measure<T: ExternalAbomonation>(typed: &T::Source) -> usize {
    use external::sealed::StructDescription;
    let slice = std::slice::from_raw_parts(std::mem::transmute(typed), std::mem::size_of::<T::Source>());
    std::mem::size_of::<T::Source>() + T::DESCRIPTION.extent(slice)
}