#![feature(prelude_import)]
#![allow(dead_code)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
use bytecheck::{check_bytes, rancor::Failure, CheckBytes};
#[repr(C)]
struct Named {
    field: bool,
}
#[automatically_derived]
unsafe impl<
    __C: ::bytecheck::rancor::Fallible + ?::core::marker::Sized,
> ::bytecheck::CheckBytes<__C> for Named
where
    <__C as ::bytecheck::rancor::Fallible>::Error: ::bytecheck::rancor::Trace,
    bool: ::bytecheck::CheckBytes<__C>,
{
    unsafe fn check_bytes(
        value: *const Self,
        context: &mut __C,
    ) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> {
        unsafe {
            <bool as ::bytecheck::CheckBytes<
                __C,
            >>::check_bytes(&raw const (*value).field, context)
        }
            .map_err(|e| {
                <<__C as ::bytecheck::rancor::Fallible>::Error as ::bytecheck::rancor::Trace>::trace(
                    e,
                    ::bytecheck::StructCheckContext {
                        struct_name: "Named",
                        field_name: "field",
                    },
                )
            })?;
        ::core::result::Result::Ok(())
    }
}
#[repr(C)]
struct Tuple(bool);
#[automatically_derived]
unsafe impl<
    __C: ::bytecheck::rancor::Fallible + ?::core::marker::Sized,
> ::bytecheck::CheckBytes<__C> for Tuple
where
    <__C as ::bytecheck::rancor::Fallible>::Error: ::bytecheck::rancor::Trace,
    bool: ::bytecheck::CheckBytes<__C>,
{
    unsafe fn check_bytes(
        value: *const Self,
        context: &mut __C,
    ) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> {
        unsafe {
            <bool as ::bytecheck::CheckBytes<
                __C,
            >>::check_bytes(&raw const (*value).0, context)
        }
            .map_err(|e| {
                <<__C as ::bytecheck::rancor::Fallible>::Error as ::bytecheck::rancor::Trace>::trace(
                    e,
                    ::bytecheck::TupleStructCheckContext {
                        tuple_struct_name: "Tuple",
                        field_index: 0usize,
                    },
                )
            })?;
        ::core::result::Result::Ok(())
    }
}
struct Unit;
#[automatically_derived]
unsafe impl<
    __C: ::bytecheck::rancor::Fallible + ?::core::marker::Sized,
> ::bytecheck::CheckBytes<__C> for Unit
where
    <__C as ::bytecheck::rancor::Fallible>::Error: ::bytecheck::rancor::Trace,
{
    unsafe fn check_bytes(
        value: *const Self,
        context: &mut __C,
    ) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> {
        ::core::result::Result::Ok(())
    }
}
#[repr(u8)]
enum Choice {
    Unit,
    Tuple(bool),
    Named { field: bool },
}
const _: () = {
    #[repr(u8)]
    enum Tag {
        Unit,
        Tuple,
        Named,
    }
    struct Discriminant;
    #[automatically_derived]
    impl Discriminant {
        #[allow(non_upper_case_globals)]
        const Unit: u8 = Tag::Unit as u8;
        #[allow(non_upper_case_globals)]
        const Tuple: u8 = Tag::Tuple as u8;
        #[allow(non_upper_case_globals)]
        const Named: u8 = Tag::Named as u8;
    }
    #[repr(C)]
    struct VariantTuple(Tag, bool, ::core::marker::PhantomData<Choice>);
    #[repr(C)]
    struct VariantNamed {
        __tag: Tag,
        field: bool,
        __phantom: ::core::marker::PhantomData<Choice>,
    }
    #[automatically_derived]
    unsafe impl<
        __C: ::bytecheck::rancor::Fallible + ?::core::marker::Sized,
    > ::bytecheck::CheckBytes<__C> for Choice
    where
        <__C as ::bytecheck::rancor::Fallible>::Error: ::bytecheck::rancor::Source,
        bool: ::bytecheck::CheckBytes<__C>,
        bool: ::bytecheck::CheckBytes<__C>,
    {
        unsafe fn check_bytes(
            value: *const Self,
            context: &mut __C,
        ) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> {
            let tag = unsafe { *value.cast::<u8>() };
            match tag {
                Discriminant::Unit => {}
                Discriminant::Tuple => {
                    let value = value.cast::<VariantTuple>();
                    unsafe {
                        <bool as ::bytecheck::CheckBytes<
                            __C,
                        >>::check_bytes(&raw const (*value).1, context)
                    }
                        .map_err(|e| {
                            <<__C as ::bytecheck::rancor::Fallible>::Error as ::bytecheck::rancor::Trace>::trace(
                                e,
                                ::bytecheck::UnnamedEnumVariantCheckContext {
                                    enum_name: "Choice",
                                    variant_name: "Tuple",
                                    field_index: 1,
                                },
                            )
                        })?;
                }
                Discriminant::Named => {
                    let value = value.cast::<VariantNamed>();
                    unsafe {
                        <bool as ::bytecheck::CheckBytes<
                            __C,
                        >>::check_bytes(&raw const (*value).field, context)
                    }
                        .map_err(|e| {
                            <<__C as ::bytecheck::rancor::Fallible>::Error as ::bytecheck::rancor::Trace>::trace(
                                e,
                                ::bytecheck::NamedEnumVariantCheckContext {
                                    enum_name: "Choice",
                                    variant_name: "Named",
                                    field_name: "field",
                                },
                            )
                        })?;
                }
                _ => {
                    return ::core::result::Result::Err(
                        <<__C as ::bytecheck::rancor::Fallible>::Error as ::bytecheck::rancor::Source>::new(::bytecheck::InvalidEnumDiscriminantError {
                            enum_name: "Choice",
                            invalid_discriminant: tag,
                        }),
                    );
                }
            }
            ::core::result::Result::Ok(())
        }
    }
};
#[repr(u8)]
enum ChoiceOneVariant {
    Boo(usize),
}
const _: () = {
    #[repr(u8)]
    enum Tag {
        Boo,
    }
    struct Discriminant;
    #[automatically_derived]
    impl Discriminant {
        #[allow(non_upper_case_globals)]
        const Boo: u8 = Tag::Boo as u8;
    }
    #[repr(C)]
    struct VariantBoo(Tag, usize, ::core::marker::PhantomData<ChoiceOneVariant>);
    #[automatically_derived]
    unsafe impl<
        __C: ::bytecheck::rancor::Fallible + ?::core::marker::Sized,
    > ::bytecheck::CheckBytes<__C> for ChoiceOneVariant
    where
        <__C as ::bytecheck::rancor::Fallible>::Error: ::bytecheck::rancor::Source,
        usize: ::bytecheck::CheckBytes<__C>,
    {
        unsafe fn check_bytes(
            value: *const Self,
            context: &mut __C,
        ) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error> {
            let tag = unsafe { *value.cast::<u8>() };
            match tag {
                Discriminant::Boo => {
                    let value = value.cast::<VariantBoo>();
                    unsafe {
                        <usize as ::bytecheck::CheckBytes<
                            __C,
                        >>::check_bytes(&raw const (*value).1, context)
                    }
                        .map_err(|e| {
                            <<__C as ::bytecheck::rancor::Fallible>::Error as ::bytecheck::rancor::Trace>::trace(
                                e,
                                ::bytecheck::UnnamedEnumVariantCheckContext {
                                    enum_name: "ChoiceOneVariant",
                                    variant_name: "Boo",
                                    field_index: 1,
                                },
                            )
                        })?;
                }
                _ => {
                    return ::core::result::Result::Err(
                        <<__C as ::bytecheck::rancor::Fallible>::Error as ::bytecheck::rancor::Source>::new(::bytecheck::InvalidEnumDiscriminantError {
                            enum_name: "ChoiceOneVariant",
                            invalid_discriminant: tag,
                        }),
                    );
                }
            }
            ::core::result::Result::Ok(())
        }
    }
};
fn main() {
    unsafe {
        check_bytes::<Named, Failure>(&Named { field: true }).unwrap();
        check_bytes::<Tuple, Failure>(&Tuple(true)).unwrap();
        check_bytes::<Unit, Failure>(&Unit).unwrap();
        check_bytes::<Choice, Failure>(&Choice::Unit).unwrap();
        check_bytes::<Choice, Failure>(&Choice::Tuple(true)).unwrap();
        check_bytes::<Choice, Failure>(&Choice::Named { field: true }).unwrap();
    }
}
