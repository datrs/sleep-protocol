// This file is generated by rust-protobuf 2.10.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `handshake.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_10_1;

#[derive(PartialEq,Clone,Default)]
pub struct Handshake {
    // message fields
    id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    live: ::std::option::Option<bool>,
    userData: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    extensions: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Handshake {
    fn default() -> &'a Handshake {
        <Handshake as ::protobuf::Message>::default_instance()
    }
}

impl Handshake {
    pub fn new() -> Handshake {
        ::std::default::Default::default()
    }

    // optional bytes id = 1;


    pub fn get_id(&self) -> &[u8] {
        match self.id.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.id.is_none() {
            self.id.set_default();
        }
        self.id.as_mut().unwrap()
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::vec::Vec<u8> {
        self.id.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    // optional bool live = 2;


    pub fn get_live(&self) -> bool {
        self.live.unwrap_or(false)
    }
    pub fn clear_live(&mut self) {
        self.live = ::std::option::Option::None;
    }

    pub fn has_live(&self) -> bool {
        self.live.is_some()
    }

    // Param is passed by value, moved
    pub fn set_live(&mut self, v: bool) {
        self.live = ::std::option::Option::Some(v);
    }

    // optional bytes userData = 3;


    pub fn get_userData(&self) -> &[u8] {
        match self.userData.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
    pub fn clear_userData(&mut self) {
        self.userData.clear();
    }

    pub fn has_userData(&self) -> bool {
        self.userData.is_some()
    }

    // Param is passed by value, moved
    pub fn set_userData(&mut self, v: ::std::vec::Vec<u8>) {
        self.userData = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_userData(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.userData.is_none() {
            self.userData.set_default();
        }
        self.userData.as_mut().unwrap()
    }

    // Take field
    pub fn take_userData(&mut self) -> ::std::vec::Vec<u8> {
        self.userData.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    // repeated string extensions = 4;


    pub fn get_extensions(&self) -> &[::std::string::String] {
        &self.extensions
    }
    pub fn clear_extensions(&mut self) {
        self.extensions.clear();
    }

    // Param is passed by value, moved
    pub fn set_extensions(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.extensions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_extensions(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.extensions
    }

    // Take field
    pub fn take_extensions(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.extensions, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for Handshake {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.live = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.userData)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.extensions)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.id.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(v) = self.live {
            my_size += 2;
        }
        if let Some(ref v) = self.userData.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        for value in &self.extensions {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.id.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(v) = self.live {
            os.write_bool(2, v)?;
        }
        if let Some(ref v) = self.userData.as_ref() {
            os.write_bytes(3, &v)?;
        }
        for v in &self.extensions {
            os.write_string(4, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Handshake {
        Handshake::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "id",
                    |m: &Handshake| { &m.id },
                    |m: &mut Handshake| { &mut m.id },
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "live",
                    |m: &Handshake| { &m.live },
                    |m: &mut Handshake| { &mut m.live },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "userData",
                    |m: &Handshake| { &m.userData },
                    |m: &mut Handshake| { &mut m.userData },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "extensions",
                    |m: &Handshake| { &m.extensions },
                    |m: &mut Handshake| { &mut m.extensions },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Handshake>(
                    "Handshake",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Handshake {
        static mut instance: ::protobuf::lazy::Lazy<Handshake> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Handshake,
        };
        unsafe {
            instance.get(Handshake::new)
        }
    }
}

impl ::protobuf::Clear for Handshake {
    fn clear(&mut self) {
        self.id.clear();
        self.live = ::std::option::Option::None;
        self.userData.clear();
        self.extensions.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Handshake {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Handshake {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef<'_> {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fhandshake.proto\x12\0\"U\n\tHandshake\x12\x0c\n\x02id\x18\x01\x20\
    \x01(\x0cB\0\x12\x0e\n\x04live\x18\x02\x20\x01(\x08B\0\x12\x12\n\x08user\
    Data\x18\x03\x20\x01(\x0cB\0\x12\x14\n\nextensions\x18\x04\x20\x03(\tB\0\
    :\0B\0b\x06proto2\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
