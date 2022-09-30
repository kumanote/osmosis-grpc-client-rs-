/// TableDescriptor describes an ORM table.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableDescriptor {
    /// primary_key defines the primary key for the table.
    #[prost(message, optional, tag = "1")]
    pub primary_key: ::core::option::Option<PrimaryKeyDescriptor>,
    /// index defines one or more secondary indexes.
    #[prost(message, repeated, tag = "2")]
    pub index: ::prost::alloc::vec::Vec<SecondaryIndexDescriptor>,
    /// id is a non-zero integer ID that must be unique within the
    /// tables and singletons in this file. It may be deprecated in the future when this
    /// can be auto-generated.
    #[prost(uint32, tag = "3")]
    pub id: u32,
}
/// PrimaryKeyDescriptor describes a table primary key.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrimaryKeyDescriptor {
    /// fields is a comma-separated list of fields in the primary key. Spaces are
    /// not allowed. Supported field types, their encodings, and any applicable constraints
    /// are described below.
    ///   - uint32 are encoded as 2,3,4 or 5 bytes using a compact encoding that
    ///     is suitable for sorted iteration (not varint encoding). This type is
    ///     well-suited for small integers.
    ///   - uint64 are encoded as 2,4,6 or 9 bytes using a compact encoding that
    ///     is suitable for sorted iteration (not varint encoding). This type is
    ///     well-suited for small integers such as auto-incrementing sequences.
    ///   - fixed32, fixed64 are encoded as big-endian fixed width bytes and support
    ///   sorted iteration. These types are well-suited for encoding fixed with
    ///   decimals as integers.
    ///   - string's are encoded as raw bytes in terminal key segments and null-terminated
    ///   in non-terminal segments. Null characters are thus forbidden in strings.
    ///   string fields support sorted iteration.
    ///   - bytes are encoded as raw bytes in terminal segments and length-prefixed
    ///   with a 32-bit unsigned varint in non-terminal segments.
    ///   - int32, sint32, int64, sint64, sfixed32, sfixed64 are encoded as fixed width bytes with
    ///   an encoding that enables sorted iteration.
    ///   - google.protobuf.Timestamp and google.protobuf.Duration are encoded
    ///   as 12 bytes using an encoding that enables sorted iteration.
    ///   - enum fields are encoded using varint encoding and do not support sorted
    ///   iteration.
    ///   - bool fields are encoded as a single byte 0 or 1.
    ///
    /// All other fields types are unsupported in keys including repeated and
    /// oneof fields.
    ///
    /// Primary keys are prefixed by the varint encoded table id and the byte 0x0
    /// plus any additional prefix specified by the schema.
    #[prost(string, tag = "1")]
    pub fields: ::prost::alloc::string::String,
    /// auto_increment specifies that the primary key is generated by an
    /// auto-incrementing integer. If this is set to true fields must only
    /// contain one field of that is of type uint64.
    #[prost(bool, tag = "2")]
    pub auto_increment: bool,
}
/// PrimaryKeyDescriptor describes a table secondary index.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecondaryIndexDescriptor {
    /// fields is a comma-separated list of fields in the index. The supported
    /// field types are the same as those for PrimaryKeyDescriptor.fields.
    /// Index keys are prefixed by the varint encoded table id and the varint
    /// encoded index id plus any additional prefix specified by the schema.
    ///
    /// In addition the field segments, non-unique index keys are suffixed with
    /// any additional primary key fields not present in the index fields so that the
    /// primary key can be reconstructed. Unique indexes instead of being suffixed
    /// store the remaining primary key fields in the value..
    #[prost(string, tag = "1")]
    pub fields: ::prost::alloc::string::String,
    /// id is a non-zero integer ID that must be unique within the indexes for this
    /// table and less than 32768. It may be deprecated in the future when this can
    /// be auto-generated.
    #[prost(uint32, tag = "2")]
    pub id: u32,
    /// unique specifies that this an unique index.
    #[prost(bool, tag = "3")]
    pub unique: bool,
}
/// TableDescriptor describes an ORM singleton table which has at most one instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SingletonDescriptor {
    /// id is a non-zero integer ID that must be unique within the
    /// tables and singletons in this file. It may be deprecated in the future when this
    /// can be auto-generated.
    #[prost(uint32, tag = "1")]
    pub id: u32,
}
