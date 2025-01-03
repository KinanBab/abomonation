use ::{encode, Abomonation};
use external::{external_encode, ExternalAbomonation};

pub fn log<T: Abomonation>(data: &T) -> String {
    let mut bytes = Vec::new();
    unsafe { encode(data, &mut bytes) }.unwrap();
    String::from_utf8_lossy(&bytes).to_string()
}

pub fn external_log<T: ExternalAbomonation>(data: &T::Source) -> String {
    let mut bytes = Vec::new();
    unsafe { external_encode::<T, _>(data, &mut bytes) }.unwrap();
    String::from_utf8_lossy(&bytes).to_string()
}