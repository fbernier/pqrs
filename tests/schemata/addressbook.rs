// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

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

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Person {
    // message fields
    pub name: ::std::string::String,
    pub id: i32,
    pub email: ::std::string::String,
    phones: ::protobuf::RepeatedField<Person_PhoneNumber>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Person {}

impl Person {
    pub fn new() -> Person {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Person {
        static mut instance: ::protobuf::lazy::Lazy<Person> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Person,
        };
        unsafe {
            instance.get(Person::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // int32 id = 2;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i32) {
        self.id = v;
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &i32 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut i32 {
        &mut self.id
    }

    // string email = 3;

    pub fn clear_email(&mut self) {
        self.email.clear();
    }

    // Param is passed by value, moved
    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_email(&mut self) -> &mut ::std::string::String {
        &mut self.email
    }

    // Take field
    pub fn take_email(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.email, ::std::string::String::new())
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    fn get_email_for_reflect(&self) -> &::std::string::String {
        &self.email
    }

    fn mut_email_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.email
    }

    // repeated .tutorial.Person.PhoneNumber phones = 4;

    pub fn clear_phones(&mut self) {
        self.phones.clear();
    }

    // Param is passed by value, moved
    pub fn set_phones(&mut self, v: ::protobuf::RepeatedField<Person_PhoneNumber>) {
        self.phones = v;
    }

    // Mutable pointer to the field.
    pub fn mut_phones(&mut self) -> &mut ::protobuf::RepeatedField<Person_PhoneNumber> {
        &mut self.phones
    }

    // Take field
    pub fn take_phones(&mut self) -> ::protobuf::RepeatedField<Person_PhoneNumber> {
        ::std::mem::replace(&mut self.phones, ::protobuf::RepeatedField::new())
    }

    pub fn get_phones(&self) -> &[Person_PhoneNumber] {
        &self.phones
    }

    fn get_phones_for_reflect(&self) -> &::protobuf::RepeatedField<Person_PhoneNumber> {
        &self.phones
    }

    fn mut_phones_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Person_PhoneNumber> {
        &mut self.phones
    }
}

impl ::protobuf::Message for Person {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.id = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.email)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.phones)?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        };
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(2, self.id, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.email.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.email);
        };
        for value in &self.phones {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        };
        if self.id != 0 {
            os.write_int32(2, self.id)?;
        };
        if !self.email.is_empty() {
            os.write_string(3, &self.email)?;
        };
        for v in &self.phones {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Person {
    fn new() -> Person {
        Person::new()
    }

    fn descriptor_static(_: ::std::option::Option<Person>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Person::get_name_for_reflect,
                    Person::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "id",
                    Person::get_id_for_reflect,
                    Person::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "email",
                    Person::get_email_for_reflect,
                    Person::mut_email_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Person_PhoneNumber>>(
                    "phones",
                    Person::get_phones_for_reflect,
                    Person::mut_phones_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Person>(
                    "Person",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Person {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_id();
        self.clear_email();
        self.clear_phones();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Person {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Person {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Person_PhoneNumber {
    // message fields
    pub number: ::std::string::String,
    pub field_type: Person_PhoneType,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Person_PhoneNumber {}

impl Person_PhoneNumber {
    pub fn new() -> Person_PhoneNumber {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Person_PhoneNumber {
        static mut instance: ::protobuf::lazy::Lazy<Person_PhoneNumber> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Person_PhoneNumber,
        };
        unsafe {
            instance.get(Person_PhoneNumber::new)
        }
    }

    // string number = 1;

    pub fn clear_number(&mut self) {
        self.number.clear();
    }

    // Param is passed by value, moved
    pub fn set_number(&mut self, v: ::std::string::String) {
        self.number = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_number(&mut self) -> &mut ::std::string::String {
        &mut self.number
    }

    // Take field
    pub fn take_number(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.number, ::std::string::String::new())
    }

    pub fn get_number(&self) -> &str {
        &self.number
    }

    fn get_number_for_reflect(&self) -> &::std::string::String {
        &self.number
    }

    fn mut_number_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.number
    }

    // .tutorial.Person.PhoneType type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type = Person_PhoneType::MOBILE;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Person_PhoneType) {
        self.field_type = v;
    }

    pub fn get_field_type(&self) -> Person_PhoneType {
        self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &Person_PhoneType {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut Person_PhoneType {
        &mut self.field_type
    }
}

impl ::protobuf::Message for Person_PhoneNumber {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.number)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.field_type = tmp;
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
        if !self.number.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.number);
        };
        if self.field_type != Person_PhoneType::MOBILE {
            my_size += ::protobuf::rt::enum_size(2, self.field_type);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.number.is_empty() {
            os.write_string(1, &self.number)?;
        };
        if self.field_type != Person_PhoneType::MOBILE {
            os.write_enum(2, self.field_type.value())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Person_PhoneNumber {
    fn new() -> Person_PhoneNumber {
        Person_PhoneNumber::new()
    }

    fn descriptor_static(_: ::std::option::Option<Person_PhoneNumber>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "number",
                    Person_PhoneNumber::get_number_for_reflect,
                    Person_PhoneNumber::mut_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Person_PhoneType>>(
                    "type",
                    Person_PhoneNumber::get_field_type_for_reflect,
                    Person_PhoneNumber::mut_field_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Person_PhoneNumber>(
                    "Person_PhoneNumber",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Person_PhoneNumber {
    fn clear(&mut self) {
        self.clear_number();
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Person_PhoneNumber {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Person_PhoneNumber {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Person_PhoneType {
    MOBILE = 0,
    HOME = 1,
    WORK = 2,
}

impl ::protobuf::ProtobufEnum for Person_PhoneType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Person_PhoneType> {
        match value {
            0 => ::std::option::Option::Some(Person_PhoneType::MOBILE),
            1 => ::std::option::Option::Some(Person_PhoneType::HOME),
            2 => ::std::option::Option::Some(Person_PhoneType::WORK),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Person_PhoneType] = &[
            Person_PhoneType::MOBILE,
            Person_PhoneType::HOME,
            Person_PhoneType::WORK,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Person_PhoneType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Person_PhoneType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Person_PhoneType {
}

impl ::std::default::Default for Person_PhoneType {
    fn default() -> Self {
        Person_PhoneType::MOBILE
    }
}

impl ::protobuf::reflect::ProtobufValue for Person_PhoneType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AddressBook {
    // message fields
    people: ::protobuf::RepeatedField<Person>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AddressBook {}

impl AddressBook {
    pub fn new() -> AddressBook {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AddressBook {
        static mut instance: ::protobuf::lazy::Lazy<AddressBook> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AddressBook,
        };
        unsafe {
            instance.get(AddressBook::new)
        }
    }

    // repeated .tutorial.Person people = 1;

    pub fn clear_people(&mut self) {
        self.people.clear();
    }

    // Param is passed by value, moved
    pub fn set_people(&mut self, v: ::protobuf::RepeatedField<Person>) {
        self.people = v;
    }

    // Mutable pointer to the field.
    pub fn mut_people(&mut self) -> &mut ::protobuf::RepeatedField<Person> {
        &mut self.people
    }

    // Take field
    pub fn take_people(&mut self) -> ::protobuf::RepeatedField<Person> {
        ::std::mem::replace(&mut self.people, ::protobuf::RepeatedField::new())
    }

    pub fn get_people(&self) -> &[Person] {
        &self.people
    }

    fn get_people_for_reflect(&self) -> &::protobuf::RepeatedField<Person> {
        &self.people
    }

    fn mut_people_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Person> {
        &mut self.people
    }
}

impl ::protobuf::Message for AddressBook {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.people)?;
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
        for value in &self.people {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.people {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AddressBook {
    fn new() -> AddressBook {
        AddressBook::new()
    }

    fn descriptor_static(_: ::std::option::Option<AddressBook>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Person>>(
                    "people",
                    AddressBook::get_people_for_reflect,
                    AddressBook::mut_people_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AddressBook>(
                    "AddressBook",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AddressBook {
    fn clear(&mut self) {
        self.clear_people();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AddressBook {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AddressBook {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x1a, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x74, 0x61, 0x2f, 0x61, 0x64, 0x64, 0x72, 0x65,
    0x73, 0x73, 0x62, 0x6f, 0x6f, 0x6b, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x08, 0x74, 0x75,
    0x74, 0x6f, 0x72, 0x69, 0x61, 0x6c, 0x22, 0xfc, 0x01, 0x0a, 0x06, 0x50, 0x65, 0x72, 0x73, 0x6f,
    0x6e, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x05, 0x52, 0x02, 0x69, 0x64, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x6d, 0x61, 0x69, 0x6c, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x65, 0x6d, 0x61, 0x69, 0x6c, 0x12, 0x34, 0x0a, 0x06, 0x70,
    0x68, 0x6f, 0x6e, 0x65, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x74, 0x75,
    0x74, 0x6f, 0x72, 0x69, 0x61, 0x6c, 0x2e, 0x50, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0x2e, 0x50, 0x68,
    0x6f, 0x6e, 0x65, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x52, 0x06, 0x70, 0x68, 0x6f, 0x6e, 0x65,
    0x73, 0x1a, 0x55, 0x0a, 0x0b, 0x50, 0x68, 0x6f, 0x6e, 0x65, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72,
    0x12, 0x16, 0x0a, 0x06, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x06, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x12, 0x2e, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1a, 0x2e, 0x74, 0x75, 0x74, 0x6f, 0x72, 0x69, 0x61,
    0x6c, 0x2e, 0x50, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0x2e, 0x50, 0x68, 0x6f, 0x6e, 0x65, 0x54, 0x79,
    0x70, 0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x22, 0x2b, 0x0a, 0x09, 0x50, 0x68, 0x6f, 0x6e,
    0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0a, 0x0a, 0x06, 0x4d, 0x4f, 0x42, 0x49, 0x4c, 0x45, 0x10,
    0x00, 0x12, 0x08, 0x0a, 0x04, 0x48, 0x4f, 0x4d, 0x45, 0x10, 0x01, 0x12, 0x08, 0x0a, 0x04, 0x57,
    0x4f, 0x52, 0x4b, 0x10, 0x02, 0x22, 0x37, 0x0a, 0x0b, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73,
    0x42, 0x6f, 0x6f, 0x6b, 0x12, 0x28, 0x0a, 0x06, 0x70, 0x65, 0x6f, 0x70, 0x6c, 0x65, 0x18, 0x01,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x74, 0x75, 0x74, 0x6f, 0x72, 0x69, 0x61, 0x6c, 0x2e,
    0x50, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0x52, 0x06, 0x70, 0x65, 0x6f, 0x70, 0x6c, 0x65, 0x42, 0x50,
    0x0a, 0x14, 0x63, 0x6f, 0x6d, 0x2e, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x2e, 0x74, 0x75,
    0x74, 0x6f, 0x72, 0x69, 0x61, 0x6c, 0x42, 0x11, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x42,
    0x6f, 0x6f, 0x6b, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0xaa, 0x02, 0x24, 0x47, 0x6f, 0x6f, 0x67,
    0x6c, 0x65, 0x2e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x45, 0x78, 0x61, 0x6d,
    0x70, 0x6c, 0x65, 0x73, 0x2e, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x42, 0x6f, 0x6f, 0x6b,
    0x4a, 0xdd, 0x0c, 0x0a, 0x06, 0x12, 0x04, 0x09, 0x00, 0x2d, 0x01, 0x0a, 0xf4, 0x02, 0x0a, 0x01,
    0x0c, 0x12, 0x03, 0x09, 0x00, 0x12, 0x1a, 0x15, 0x20, 0x5b, 0x53, 0x54, 0x41, 0x52, 0x54, 0x20,
    0x64, 0x65, 0x63, 0x6c, 0x61, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5d, 0x0a, 0x32, 0xd2, 0x02,
    0x20, 0x53, 0x65, 0x65, 0x20, 0x52, 0x45, 0x41, 0x44, 0x4d, 0x45, 0x2e, 0x74, 0x78, 0x74, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20,
    0x61, 0x6e, 0x64, 0x20, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x20, 0x69, 0x6e, 0x73, 0x74, 0x72, 0x75,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x0a, 0x0a, 0x20, 0x4e, 0x6f, 0x74, 0x65, 0x3a, 0x20,
    0x53, 0x54, 0x41, 0x52, 0x54, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x45, 0x4e, 0x44, 0x20, 0x74, 0x61,
    0x67, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x63,
    0x6f, 0x6d, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e,
    0x65, 0x20, 0x73, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20,
    0x69, 0x6e, 0x0a, 0x20, 0x74, 0x75, 0x74, 0x6f, 0x72, 0x69, 0x61, 0x6c, 0x73, 0x2e, 0x20, 0x20,
    0x54, 0x68, 0x65, 0x79, 0x20, 0x61, 0x72, 0x65, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x70, 0x61, 0x72,
    0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x79, 0x6e, 0x74, 0x61, 0x78, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x20, 0x42, 0x75, 0x66,
    0x66, 0x65, 0x72, 0x73, 0x2e, 0x0a, 0x0a, 0x20, 0x54, 0x6f, 0x20, 0x67, 0x65, 0x74, 0x20, 0x61,
    0x6e, 0x20, 0x69, 0x6e, 0x2d, 0x64, 0x65, 0x70, 0x74, 0x68, 0x20, 0x77, 0x61, 0x6c, 0x6b, 0x74,
    0x68, 0x72, 0x6f, 0x75, 0x67, 0x68, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x66,
    0x69, 0x6c, 0x65, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x6c, 0x61,
    0x74, 0x65, 0x64, 0x20, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x73, 0x2c, 0x20, 0x73, 0x65,
    0x65, 0x3a, 0x0a, 0x20, 0x68, 0x74, 0x74, 0x70, 0x73, 0x3a, 0x2f, 0x2f, 0x64, 0x65, 0x76, 0x65,
    0x6c, 0x6f, 0x70, 0x65, 0x72, 0x73, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x63, 0x6f,
    0x6d, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x2d, 0x62, 0x75, 0x66, 0x66, 0x65,
    0x72, 0x73, 0x2f, 0x64, 0x6f, 0x63, 0x73, 0x2f, 0x74, 0x75, 0x74, 0x6f, 0x72, 0x69, 0x61, 0x6c,
    0x73, 0x0a, 0x0a, 0x1d, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x0a, 0x08, 0x10, 0x22, 0x13, 0x20, 0x5b,
    0x45, 0x4e, 0x44, 0x20, 0x64, 0x65, 0x63, 0x6c, 0x61, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5d,
    0x0a, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0e, 0x00, 0x2d, 0x0a, 0x27, 0x0a, 0x04, 0x08,
    0xe7, 0x07, 0x00, 0x12, 0x03, 0x0e, 0x00, 0x2d, 0x1a, 0x1a, 0x20, 0x5b, 0x53, 0x54, 0x41, 0x52,
    0x54, 0x20, 0x6a, 0x61, 0x76, 0x61, 0x5f, 0x64, 0x65, 0x63, 0x6c, 0x61, 0x72, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x5d, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x0e,
    0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0e, 0x07,
    0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x07,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x07, 0x12, 0x03, 0x0e, 0x16, 0x2c, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0f, 0x00, 0x32, 0x0a, 0x25, 0x0a, 0x04, 0x08, 0xe7, 0x07,
    0x01, 0x12, 0x03, 0x0f, 0x00, 0x32, 0x22, 0x18, 0x20, 0x5b, 0x45, 0x4e, 0x44, 0x20, 0x6a, 0x61,
    0x76, 0x61, 0x5f, 0x64, 0x65, 0x63, 0x6c, 0x61, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5d, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x12, 0x03, 0x0f, 0x07, 0x1b, 0x0a, 0x0d,
    0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0f, 0x07, 0x1b, 0x0a, 0x0e, 0x0a,
    0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0f, 0x07, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x08, 0xe7, 0x07, 0x01, 0x07, 0x12, 0x03, 0x0f, 0x1e, 0x31, 0x0a, 0x08, 0x0a, 0x01, 0x08,
    0x12, 0x03, 0x13, 0x00, 0x41, 0x0a, 0x45, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x02, 0x12, 0x03, 0x13,
    0x00, 0x41, 0x1a, 0x1c, 0x20, 0x5b, 0x53, 0x54, 0x41, 0x52, 0x54, 0x20, 0x63, 0x73, 0x68, 0x61,
    0x72, 0x70, 0x5f, 0x64, 0x65, 0x63, 0x6c, 0x61, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5d, 0x0a,
    0x22, 0x1a, 0x20, 0x5b, 0x45, 0x4e, 0x44, 0x20, 0x63, 0x73, 0x68, 0x61, 0x72, 0x70, 0x5f, 0x64,
    0x65, 0x63, 0x6c, 0x61, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5d, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x08, 0xe7, 0x07, 0x02, 0x02, 0x12, 0x03, 0x13, 0x07, 0x17, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7,
    0x07, 0x02, 0x02, 0x00, 0x12, 0x03, 0x13, 0x07, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07,
    0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x13, 0x07, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07,
    0x02, 0x07, 0x12, 0x03, 0x13, 0x1a, 0x40, 0x0a, 0x1e, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x17,
    0x00, 0x28, 0x01, 0x1a, 0x12, 0x20, 0x5b, 0x53, 0x54, 0x41, 0x52, 0x54, 0x20, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x73, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x17, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x18, 0x02, 0x12,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x18, 0x02, 0x17, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x18, 0x02, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x18, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x18, 0x10, 0x11, 0x0a, 0x30, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x01, 0x12, 0x03, 0x19, 0x02, 0x0f, 0x22, 0x23, 0x20, 0x55, 0x6e, 0x69, 0x71, 0x75, 0x65, 0x20,
    0x49, 0x44, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68,
    0x69, 0x73, 0x20, 0x70, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x19, 0x02, 0x18, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x19, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x19, 0x08, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x19, 0x0d, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x1a, 0x02,
    0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0x1a, 0x02, 0x19, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x1a, 0x02, 0x08, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1a, 0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1a, 0x11, 0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00,
    0x04, 0x00, 0x12, 0x04, 0x1c, 0x02, 0x20, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x1c, 0x07, 0x10, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x1d, 0x04, 0x0f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x1d, 0x04, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x02,
    0x12, 0x03, 0x1d, 0x0d, 0x0e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x1e, 0x04, 0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x1e, 0x04, 0x08, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12,
    0x03, 0x1e, 0x0b, 0x0c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03,
    0x1f, 0x04, 0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x1f, 0x04, 0x08, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03,
    0x1f, 0x0b, 0x0c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x03, 0x00, 0x12, 0x04, 0x22, 0x02, 0x25,
    0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x00, 0x01, 0x12, 0x03, 0x22, 0x0a, 0x15, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x23, 0x04, 0x16, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x23, 0x04, 0x22, 0x17, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x23, 0x04, 0x0a, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x23, 0x0b, 0x11, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x23, 0x14, 0x15, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x24, 0x04, 0x17, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x24, 0x04, 0x23, 0x16, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x24, 0x04, 0x0d, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x24, 0x0e, 0x12, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x24, 0x15, 0x16, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x27, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x27, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x06, 0x12, 0x03, 0x27, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x27, 0x17, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x27, 0x20, 0x21, 0x0a, 0x39, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x2b, 0x00, 0x2d, 0x01,
    0x1a, 0x2d, 0x20, 0x4f, 0x75, 0x72, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x62,
    0x6f, 0x6f, 0x6b, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20, 0x69, 0x73, 0x20, 0x6a, 0x75, 0x73, 0x74,
    0x20, 0x6f, 0x6e, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x73, 0x65, 0x2e, 0x0a, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x2b, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x00, 0x12, 0x03, 0x2c, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x2c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x2c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2c,
    0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2c, 0x1b, 0x1c,
    0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];

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