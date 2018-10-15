/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

use toml;

#[derive(Debug)]
/// Possible errors that can occur during Cargo.toml parsing.
pub enum Error {
    /// Error during reading of Cargo.toml
    Io(io::Error),
    /// Deserialization error
    Toml(toml::de::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}
impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::Toml(err)
    }
}

// Warning: Do not change this without regenerating serde_derive impls.
// See issue #203 and README.serde_derive for more information.
#[derive(Clone, Debug)]
pub struct Lock {
    pub root: Option<Package>,
    pub package: Option<Vec<Package>>,
}

// Warning: Do not change this without regenerating serde_derive impls.
// See issue #203 and README.serde_derive for more information.
#[derive(Clone, Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    /// A list of dependencies formatted like "NAME VERSION REGISTRY-OPT"
    pub dependencies: Option<Vec<String>>,
}

/// Parse the Cargo.toml for a given path
pub fn lock(manifest_path: &Path) -> Result<Lock, Error> {
    let mut s = String::new();
    let mut f = File::open(manifest_path)?;
    f.read_to_string(&mut s)?;

    toml::from_str::<Lock>(&s).map_err(|x| x.into())
}

// Warning: The following code is autogenerated by serde_derive, don't touch
// unless you know what you're doing. See issue #203 and README.serde_derive
// for more information.

// Generated from `serde_derive 1.80.0`

#[cfg_attr(rustfmt, rustfmt_skip)]
#[allow(non_upper_case_globals,
        unused_attributes,
        unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_Lock: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                         match $ __expr {
                         _serde :: export :: Ok ( __val ) => __val
                         , _serde :: export :: Err ( __err ) => {
                         return _serde :: export :: Err ( __err )
                         ; } } });
        #[automatically_derived]
        impl <'de> _serde::Deserialize<'de> for Lock {
            fn deserialize<__D>(__deserializer: __D)
             -> _serde::export::Result<Self, __D::Error> where
             __D: _serde::Deserializer<'de> {
                #[allow(non_camel_case_types)]
                enum __Field { __field0, __field1, __ignore, }
                struct __FieldVisitor;
                impl <'de> _serde::de::Visitor<'de> for
                 __FieldVisitor {
                    type
                    Value
                    =
                    __Field;
                    fn expecting(&self,
                                 __formatter:
                                     &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "field identifier")
                    }
                    fn visit_u64<__E>(self, __value: u64)
                     -> _serde::export::Result<Self::Value, __E>
                     where __E: _serde::de::Error {
                        match __value {
                            0u64 =>
                            _serde::export::Ok(__Field::__field0),
                            1u64 =>
                            _serde::export::Ok(__Field::__field1),
                            _ =>
                            _serde::export::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                 &"field index 0 <= i < 2")),
                        }
                    }
                    fn visit_str<__E>(self, __value: &str)
                     -> _serde::export::Result<Self::Value, __E>
                     where __E: _serde::de::Error {
                        match __value {
                            "root" =>
                            _serde::export::Ok(__Field::__field0),
                            "package" =>
                            _serde::export::Ok(__Field::__field1),
                            _ => {
                                _serde::export::Ok(__Field::__ignore)
                            }
                        }
                    }
                    fn visit_bytes<__E>(self, __value: &[u8])
                     -> _serde::export::Result<Self::Value, __E>
                     where __E: _serde::de::Error {
                        match __value {
                            b"root" =>
                            _serde::export::Ok(__Field::__field0),
                            b"package" =>
                            _serde::export::Ok(__Field::__field1),
                            _ => {
                                _serde::export::Ok(__Field::__ignore)
                            }
                        }
                    }
                }
                impl <'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(__deserializer: __D)
                     -> _serde::export::Result<Self, __D::Error>
                     where __D: _serde::Deserializer<'de> {
                        _serde::Deserializer::deserialize_identifier(__deserializer,
                                                                     __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<Lock>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de> _serde::de::Visitor<'de> for
                 __Visitor<'de> {
                    type
                    Value
                    =
                    Lock;
                    fn expecting(&self,
                                 __formatter:
                                     &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "struct Lock")
                    }
                    #[inline]
                    fn visit_seq<__A>(self, mut __seq: __A)
                     ->
                         _serde::export::Result<Self::Value,
                                                __A::Error> where
                     __A: _serde::de::SeqAccess<'de> {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<Option<Package>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err)
                                      => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                 &"struct Lock with 2 elements"));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<Option<Vec<Package>>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err)
                                      => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(1usize,
                                                                                                 &"struct Lock with 2 elements"));
                                }
                            };
                        _serde::export::Ok(Lock{root: __field0,
                                                package:
                                                    __field1,})
                    }
                    #[inline]
                    fn visit_map<__A>(self, mut __map: __A)
                     ->
                         _serde::export::Result<Self::Value,
                                                __A::Error> where
                     __A: _serde::de::MapAccess<'de> {
                        let mut __field0:
                                _serde::export::Option<Option<Package>> =
                            _serde::export::None;
                        let mut __field1:
                                _serde::export::Option<Option<Vec<Package>>> =
                            _serde::export::None;
                        while let _serde::export::Some(__key) =
                                  match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err)
                                      => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("root"));
                                    }
                                    __field0 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<Package>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 =>
                                                                 {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("package"));
                                    }
                                    __field1 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<Vec<Package>>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 =>
                                                                 {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                _ => {
                                    let _ =
                                        match _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)
                                            {
                                            _serde::export::Ok(__val)
                                            => __val,
                                            _serde::export::Err(__err)
                                            => {
                                                return _serde::export::Err(__err);
                                            }
                                        };
                                }
                            }
                        }
                        let __field0 =
                            match __field0 {
                                _serde::export::Some(__field0) =>
                                __field0,
                                _serde::export::None =>
                                match _serde::private::de::missing_field("root")
                                    {
                                    _serde::export::Ok(__val) =>
                                    __val,
                                    _serde::export::Err(__err) =>
                                    {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        let __field1 =
                            match __field1 {
                                _serde::export::Some(__field1) =>
                                __field1,
                                _serde::export::None =>
                                match _serde::private::de::missing_field("package")
                                    {
                                    _serde::export::Ok(__val) =>
                                    __val,
                                    _serde::export::Err(__err) =>
                                    {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        _serde::export::Ok(Lock{root: __field0,
                                                package:
                                                    __field1,})
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["root", "package"];
                _serde::Deserializer::deserialize_struct(__deserializer,
                                                         "Lock",
                                                         FIELDS,
                                                         __Visitor{marker:
                                                                       _serde::export::PhantomData::<Lock>,
                                                                   lifetime:
                                                                       _serde::export::PhantomData,})
            }
        }
    };

#[cfg_attr(rustfmt, rustfmt_skip)]
#[allow(non_upper_case_globals,
        unused_attributes,
        unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_Package: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                         match $ __expr {
                         _serde :: export :: Ok ( __val ) => __val
                         , _serde :: export :: Err ( __err ) => {
                         return _serde :: export :: Err ( __err )
                         ; } } });
        #[automatically_derived]
        impl <'de> _serde::Deserialize<'de> for Package {
            fn deserialize<__D>(__deserializer: __D)
             -> _serde::export::Result<Self, __D::Error> where
             __D: _serde::Deserializer<'de> {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                struct __FieldVisitor;
                impl <'de> _serde::de::Visitor<'de> for
                 __FieldVisitor {
                    type
                    Value
                    =
                    __Field;
                    fn expecting(&self,
                                 __formatter:
                                     &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "field identifier")
                    }
                    fn visit_u64<__E>(self, __value: u64)
                     -> _serde::export::Result<Self::Value, __E>
                     where __E: _serde::de::Error {
                        match __value {
                            0u64 =>
                            _serde::export::Ok(__Field::__field0),
                            1u64 =>
                            _serde::export::Ok(__Field::__field1),
                            2u64 =>
                            _serde::export::Ok(__Field::__field2),
                            _ =>
                            _serde::export::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                 &"field index 0 <= i < 3")),
                        }
                    }
                    fn visit_str<__E>(self, __value: &str)
                     -> _serde::export::Result<Self::Value, __E>
                     where __E: _serde::de::Error {
                        match __value {
                            "name" =>
                            _serde::export::Ok(__Field::__field0),
                            "version" =>
                            _serde::export::Ok(__Field::__field1),
                            "dependencies" =>
                            _serde::export::Ok(__Field::__field2),
                            _ => {
                                _serde::export::Ok(__Field::__ignore)
                            }
                        }
                    }
                    fn visit_bytes<__E>(self, __value: &[u8])
                     -> _serde::export::Result<Self::Value, __E>
                     where __E: _serde::de::Error {
                        match __value {
                            b"name" =>
                            _serde::export::Ok(__Field::__field0),
                            b"version" =>
                            _serde::export::Ok(__Field::__field1),
                            b"dependencies" =>
                            _serde::export::Ok(__Field::__field2),
                            _ => {
                                _serde::export::Ok(__Field::__ignore)
                            }
                        }
                    }
                }
                impl <'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(__deserializer: __D)
                     -> _serde::export::Result<Self, __D::Error>
                     where __D: _serde::Deserializer<'de> {
                        _serde::Deserializer::deserialize_identifier(__deserializer,
                                                                     __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<Package>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de> _serde::de::Visitor<'de> for
                 __Visitor<'de> {
                    type
                    Value
                    =
                    Package;
                    fn expecting(&self,
                                 __formatter:
                                     &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "struct Package")
                    }
                    #[inline]
                    fn visit_seq<__A>(self, mut __seq: __A)
                     ->
                         _serde::export::Result<Self::Value,
                                                __A::Error> where
                     __A: _serde::de::SeqAccess<'de> {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err)
                                      => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                 &"struct Package with 3 elements"));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<String>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err)
                                      => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(1usize,
                                                                                                 &"struct Package with 3 elements"));
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<Option<Vec<String>>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err)
                                      => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) =>
                                __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(2usize,
                                                                                                 &"struct Package with 3 elements"));
                                }
                            };
                        _serde::export::Ok(Package{name: __field0,
                                                   version:
                                                       __field1,
                                                   dependencies:
                                                       __field2,})
                    }
                    #[inline]
                    fn visit_map<__A>(self, mut __map: __A)
                     ->
                         _serde::export::Result<Self::Value,
                                                __A::Error> where
                     __A: _serde::de::MapAccess<'de> {
                        let mut __field0:
                                _serde::export::Option<String> =
                            _serde::export::None;
                        let mut __field1:
                                _serde::export::Option<String> =
                            _serde::export::None;
                        let mut __field2:
                                _serde::export::Option<Option<Vec<String>>> =
                            _serde::export::None;
                        while let _serde::export::Some(__key) =
                                  match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                      {
                                      _serde::export::Ok(__val) =>
                                      __val,
                                      _serde::export::Err(__err)
                                      => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("name"));
                                    }
                                    __field0 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<String>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 =>
                                                                 {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("version"));
                                    }
                                    __field1 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<String>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 =>
                                                                 {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("dependencies"));
                                    }
                                    __field2 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<Vec<String>>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 =>
                                                                 __val,
                                                                 _serde::export::Err(__err)
                                                                 =>
                                                                 {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                _ => {
                                    let _ =
                                        match _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)
                                            {
                                            _serde::export::Ok(__val)
                                            => __val,
                                            _serde::export::Err(__err)
                                            => {
                                                return _serde::export::Err(__err);
                                            }
                                        };
                                }
                            }
                        }
                        let __field0 =
                            match __field0 {
                                _serde::export::Some(__field0) =>
                                __field0,
                                _serde::export::None =>
                                match _serde::private::de::missing_field("name")
                                    {
                                    _serde::export::Ok(__val) =>
                                    __val,
                                    _serde::export::Err(__err) =>
                                    {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        let __field1 =
                            match __field1 {
                                _serde::export::Some(__field1) =>
                                __field1,
                                _serde::export::None =>
                                match _serde::private::de::missing_field("version")
                                    {
                                    _serde::export::Ok(__val) =>
                                    __val,
                                    _serde::export::Err(__err) =>
                                    {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        let __field2 =
                            match __field2 {
                                _serde::export::Some(__field2) =>
                                __field2,
                                _serde::export::None =>
                                match _serde::private::de::missing_field("dependencies")
                                    {
                                    _serde::export::Ok(__val) =>
                                    __val,
                                    _serde::export::Err(__err) =>
                                    {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        _serde::export::Ok(Package{name: __field0,
                                                   version:
                                                       __field1,
                                                   dependencies:
                                                       __field2,})
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["name", "version", "dependencies"];
                _serde::Deserializer::deserialize_struct(__deserializer,
                                                         "Package",
                                                         FIELDS,
                                                         __Visitor{marker:
                                                                       _serde::export::PhantomData::<Package>,
                                                                   lifetime:
                                                                       _serde::export::PhantomData,})
            }
        }
    };
