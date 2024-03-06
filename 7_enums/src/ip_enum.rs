
pub enum IpAddrType{
    V4 = 4,
    V6 = 6
}
pub struct Ipv6Addr {
    addr: String
}
pub enum IP{
    V4(u8,u8,u8,u8),
    V6(Ipv6Addr) // Can also contain a struct. Actually it can contain any types
}

pub struct IpAddr {
    pub(crate) ip_type: IpAddrType,
    pub(crate) ip : String,
}

pub fn route(ip_kind:IpAddrType){}