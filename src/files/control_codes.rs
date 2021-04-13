/*
Group Separator. Separates entries in header. <char><freq><GS><char><freq>...
GS code is 29, but I use 0 for safety, because it's guarantied, that there won't be 0 frequency char
So if it's 0 in the header it's 100% Group Separator
*/
pub const GS: u8 = 0;
// Unit Separator. Separates header and compressed data
pub const US: u8 = 31;
// End Of Text. Defines end of data.
pub const EOT: u8 = 3;
