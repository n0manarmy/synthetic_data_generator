use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Debug)]
pub struct ResolveObject {
    location: &'static str,
    ip: String,
    asn: &'static str,
}

impl ResolveObject {
    const _LOCATION_LABEL: &'static str = "geo";
    const _IP_LABEL: &'static str = "ip";
    const _ASN_LABEL: &'static str = "asn";

    pub fn new(location: &'static str, ip: String, asn: &'static str) -> ResolveObject {
        ResolveObject { location, ip, asn }
    }
}

impl Serialize for ResolveObject {
    fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
    where
        T: Serializer,
    {
        let mut state = serializer.serialize_struct("resolve object", 3)?;
        state.serialize_field(Self::_LOCATION_LABEL, self.location)?;
        state.serialize_field(Self::_IP_LABEL, &self.ip)?;
        state.serialize_field(Self::_ASN_LABEL, self.asn)?;
        state.end()
    }
}
