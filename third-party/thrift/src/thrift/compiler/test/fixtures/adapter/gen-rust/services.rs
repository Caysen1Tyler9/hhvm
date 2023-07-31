// @generated by Thrift for thrift/compiler/test/fixtures/adapter/src/module.thrift
// This file is probably not the place you want to edit!

//! Thrift service definitions for `module`.


/// Service definitions for `Service`.
pub mod service {
    #[derive(Clone, Debug)]
    pub enum FuncExn {
        #[doc(hidden)]
        Success(crate::types::MyI32_4873),
        ApplicationException(::fbthrift::ApplicationException),
    }

    impl ::std::convert::From<crate::errors::service::FuncError> for FuncExn {
        fn from(err: crate::errors::service::FuncError) -> Self {
            match err {
                crate::errors::service::FuncError::ApplicationException(aexn) => FuncExn::ApplicationException(aexn),
                crate::errors::service::FuncError::ThriftError(err) => FuncExn::ApplicationException(::fbthrift::ApplicationException {
                    message: err.to_string(),
                    type_: ::fbthrift::ApplicationExceptionErrorCode::InternalError,
                }),
            }
        }
    }

    impl ::std::convert::From<::fbthrift::ApplicationException> for FuncExn {
        fn from(exn: ::fbthrift::ApplicationException) -> Self {
            Self::ApplicationException(exn)
        }
    }

    impl ::fbthrift::ExceptionInfo for FuncExn {
        fn exn_name(&self) -> &'static str {
            match self {
                Self::Success(_) => panic!("ExceptionInfo::exn_name called on Success"),
                Self::ApplicationException(aexn) => aexn.exn_name(),
            }
        }

        fn exn_value(&self) -> String {
            match self {
                Self::Success(_) => panic!("ExceptionInfo::exn_value called on Success"),
                Self::ApplicationException(aexn) => aexn.exn_value(),
            }
        }

        fn exn_is_declared(&self) -> bool {
            match self {
                Self::Success(_) => panic!("ExceptionInfo::exn_is_declared called on Success"),
                Self::ApplicationException(aexn) => aexn.exn_is_declared(),
            }
        }
    }

    impl ::fbthrift::ResultInfo for FuncExn {
        fn result_type(&self) -> ::fbthrift::ResultType {
            match self {
                Self::Success(_) => ::fbthrift::ResultType::Return,
                Self::ApplicationException(_aexn) => ::fbthrift::ResultType::Exception,
            }
        }
    }

    impl ::fbthrift::GetTType for FuncExn {
        const TTYPE: ::fbthrift::TType = ::fbthrift::TType::Struct;
    }

    impl<P> ::fbthrift::Serialize<P> for FuncExn
    where
        P: ::fbthrift::ProtocolWriter,
    {
        fn write(&self, p: &mut P) {
            if let Self::ApplicationException(aexn) = self {
                return aexn.write(p);
            }
            p.write_struct_begin("Func");
            match self {
                Self::Success(inner) => {
                    p.write_field_begin(
                        "Success",
                        ::fbthrift::TType::I32,
                        0i16,
                    );
                    inner.write(p);
                    p.write_field_end();
                }
                Self::ApplicationException(_aexn) => unreachable!(),
            }
            p.write_field_stop();
            p.write_struct_end();
        }
    }

    impl<P> ::fbthrift::Deserialize<P> for FuncExn
    where
        P: ::fbthrift::ProtocolReader,
    {
        fn read(p: &mut P) -> ::anyhow::Result<Self> {
            static RETURNS: &[::fbthrift::Field] = &[
                ::fbthrift::Field::new("Success", ::fbthrift::TType::I32, 0),
            ];
            let _ = p.read_struct_begin(|_| ())?;
            let mut once = false;
            let mut alt = ::std::option::Option::None;
            loop {
                let (_, fty, fid) = p.read_field_begin(|_| (), RETURNS)?;
                match ((fty, fid as ::std::primitive::i32), once) {
                    ((::fbthrift::TType::Stop, _), _) => {
                        p.read_field_end()?;
                        break;
                    }
                    ((::fbthrift::TType::I32, 0i32), false) => {
                        once = true;
                        alt = ::std::option::Option::Some(Self::Success(::fbthrift::Deserialize::read(p)?));
                    }
                    ((ty, _id), false) => p.skip(ty)?,
                    ((badty, badid), true) => return ::std::result::Result::Err(::std::convert::From::from(
                        ::fbthrift::ApplicationException::new(
                            ::fbthrift::ApplicationExceptionErrorCode::ProtocolError,
                            format!(
                                "unwanted extra union {} field ty {:?} id {}",
                                "FuncExn",
                                badty,
                                badid,
                            ),
                        )
                    )),
                }
                p.read_field_end()?;
            }
            p.read_struct_end()?;
            alt.ok_or_else(||
                ::fbthrift::ApplicationException::new(
                    ::fbthrift::ApplicationExceptionErrorCode::MissingResult,
                    format!("Empty union {}", "FuncExn"),
                )
                .into(),
            )
        }
    }
}

/// Service definitions for `AdapterService`.
pub mod adapter_service {
    #[derive(Clone, Debug)]
    pub enum CountExn {
        #[doc(hidden)]
        Success(crate::types::CountingStruct),
        ApplicationException(::fbthrift::ApplicationException),
    }

    impl ::std::convert::From<crate::errors::adapter_service::CountError> for CountExn {
        fn from(err: crate::errors::adapter_service::CountError) -> Self {
            match err {
                crate::errors::adapter_service::CountError::ApplicationException(aexn) => CountExn::ApplicationException(aexn),
                crate::errors::adapter_service::CountError::ThriftError(err) => CountExn::ApplicationException(::fbthrift::ApplicationException {
                    message: err.to_string(),
                    type_: ::fbthrift::ApplicationExceptionErrorCode::InternalError,
                }),
            }
        }
    }

    impl ::std::convert::From<::fbthrift::ApplicationException> for CountExn {
        fn from(exn: ::fbthrift::ApplicationException) -> Self {
            Self::ApplicationException(exn)
        }
    }

    impl ::fbthrift::ExceptionInfo for CountExn {
        fn exn_name(&self) -> &'static str {
            match self {
                Self::Success(_) => panic!("ExceptionInfo::exn_name called on Success"),
                Self::ApplicationException(aexn) => aexn.exn_name(),
            }
        }

        fn exn_value(&self) -> String {
            match self {
                Self::Success(_) => panic!("ExceptionInfo::exn_value called on Success"),
                Self::ApplicationException(aexn) => aexn.exn_value(),
            }
        }

        fn exn_is_declared(&self) -> bool {
            match self {
                Self::Success(_) => panic!("ExceptionInfo::exn_is_declared called on Success"),
                Self::ApplicationException(aexn) => aexn.exn_is_declared(),
            }
        }
    }

    impl ::fbthrift::ResultInfo for CountExn {
        fn result_type(&self) -> ::fbthrift::ResultType {
            match self {
                Self::Success(_) => ::fbthrift::ResultType::Return,
                Self::ApplicationException(_aexn) => ::fbthrift::ResultType::Exception,
            }
        }
    }

    impl ::fbthrift::GetTType for CountExn {
        const TTYPE: ::fbthrift::TType = ::fbthrift::TType::Struct;
    }

    impl<P> ::fbthrift::Serialize<P> for CountExn
    where
        P: ::fbthrift::ProtocolWriter,
    {
        fn write(&self, p: &mut P) {
            if let Self::ApplicationException(aexn) = self {
                return aexn.write(p);
            }
            p.write_struct_begin("Count");
            match self {
                Self::Success(inner) => {
                    p.write_field_begin(
                        "Success",
                        ::fbthrift::TType::Struct,
                        0i16,
                    );
                    inner.write(p);
                    p.write_field_end();
                }
                Self::ApplicationException(_aexn) => unreachable!(),
            }
            p.write_field_stop();
            p.write_struct_end();
        }
    }

    impl<P> ::fbthrift::Deserialize<P> for CountExn
    where
        P: ::fbthrift::ProtocolReader,
    {
        fn read(p: &mut P) -> ::anyhow::Result<Self> {
            static RETURNS: &[::fbthrift::Field] = &[
                ::fbthrift::Field::new("Success", ::fbthrift::TType::Struct, 0),
            ];
            let _ = p.read_struct_begin(|_| ())?;
            let mut once = false;
            let mut alt = ::std::option::Option::None;
            loop {
                let (_, fty, fid) = p.read_field_begin(|_| (), RETURNS)?;
                match ((fty, fid as ::std::primitive::i32), once) {
                    ((::fbthrift::TType::Stop, _), _) => {
                        p.read_field_end()?;
                        break;
                    }
                    ((::fbthrift::TType::Struct, 0i32), false) => {
                        once = true;
                        alt = ::std::option::Option::Some(Self::Success(::fbthrift::Deserialize::read(p)?));
                    }
                    ((ty, _id), false) => p.skip(ty)?,
                    ((badty, badid), true) => return ::std::result::Result::Err(::std::convert::From::from(
                        ::fbthrift::ApplicationException::new(
                            ::fbthrift::ApplicationExceptionErrorCode::ProtocolError,
                            format!(
                                "unwanted extra union {} field ty {:?} id {}",
                                "CountExn",
                                badty,
                                badid,
                            ),
                        )
                    )),
                }
                p.read_field_end()?;
            }
            p.read_struct_end()?;
            alt.ok_or_else(||
                ::fbthrift::ApplicationException::new(
                    ::fbthrift::ApplicationExceptionErrorCode::MissingResult,
                    format!("Empty union {}", "CountExn"),
                )
                .into(),
            )
        }
    }

    #[derive(Clone, Debug)]
    pub enum AdaptedTypesExn {
        #[doc(hidden)]
        Success(crate::types::HeapAllocated),
        ApplicationException(::fbthrift::ApplicationException),
    }

    impl ::std::convert::From<crate::errors::adapter_service::AdaptedTypesError> for AdaptedTypesExn {
        fn from(err: crate::errors::adapter_service::AdaptedTypesError) -> Self {
            match err {
                crate::errors::adapter_service::AdaptedTypesError::ApplicationException(aexn) => AdaptedTypesExn::ApplicationException(aexn),
                crate::errors::adapter_service::AdaptedTypesError::ThriftError(err) => AdaptedTypesExn::ApplicationException(::fbthrift::ApplicationException {
                    message: err.to_string(),
                    type_: ::fbthrift::ApplicationExceptionErrorCode::InternalError,
                }),
            }
        }
    }

    impl ::std::convert::From<::fbthrift::ApplicationException> for AdaptedTypesExn {
        fn from(exn: ::fbthrift::ApplicationException) -> Self {
            Self::ApplicationException(exn)
        }
    }

    impl ::fbthrift::ExceptionInfo for AdaptedTypesExn {
        fn exn_name(&self) -> &'static str {
            match self {
                Self::Success(_) => panic!("ExceptionInfo::exn_name called on Success"),
                Self::ApplicationException(aexn) => aexn.exn_name(),
            }
        }

        fn exn_value(&self) -> String {
            match self {
                Self::Success(_) => panic!("ExceptionInfo::exn_value called on Success"),
                Self::ApplicationException(aexn) => aexn.exn_value(),
            }
        }

        fn exn_is_declared(&self) -> bool {
            match self {
                Self::Success(_) => panic!("ExceptionInfo::exn_is_declared called on Success"),
                Self::ApplicationException(aexn) => aexn.exn_is_declared(),
            }
        }
    }

    impl ::fbthrift::ResultInfo for AdaptedTypesExn {
        fn result_type(&self) -> ::fbthrift::ResultType {
            match self {
                Self::Success(_) => ::fbthrift::ResultType::Return,
                Self::ApplicationException(_aexn) => ::fbthrift::ResultType::Exception,
            }
        }
    }

    impl ::fbthrift::GetTType for AdaptedTypesExn {
        const TTYPE: ::fbthrift::TType = ::fbthrift::TType::Struct;
    }

    impl<P> ::fbthrift::Serialize<P> for AdaptedTypesExn
    where
        P: ::fbthrift::ProtocolWriter,
    {
        fn write(&self, p: &mut P) {
            if let Self::ApplicationException(aexn) = self {
                return aexn.write(p);
            }
            p.write_struct_begin("AdaptedTypes");
            match self {
                Self::Success(inner) => {
                    p.write_field_begin(
                        "Success",
                        ::fbthrift::TType::Struct,
                        0i16,
                    );
                    inner.write(p);
                    p.write_field_end();
                }
                Self::ApplicationException(_aexn) => unreachable!(),
            }
            p.write_field_stop();
            p.write_struct_end();
        }
    }

    impl<P> ::fbthrift::Deserialize<P> for AdaptedTypesExn
    where
        P: ::fbthrift::ProtocolReader,
    {
        fn read(p: &mut P) -> ::anyhow::Result<Self> {
            static RETURNS: &[::fbthrift::Field] = &[
                ::fbthrift::Field::new("Success", ::fbthrift::TType::Struct, 0),
            ];
            let _ = p.read_struct_begin(|_| ())?;
            let mut once = false;
            let mut alt = ::std::option::Option::None;
            loop {
                let (_, fty, fid) = p.read_field_begin(|_| (), RETURNS)?;
                match ((fty, fid as ::std::primitive::i32), once) {
                    ((::fbthrift::TType::Stop, _), _) => {
                        p.read_field_end()?;
                        break;
                    }
                    ((::fbthrift::TType::Struct, 0i32), false) => {
                        once = true;
                        alt = ::std::option::Option::Some(Self::Success(::fbthrift::Deserialize::read(p)?));
                    }
                    ((ty, _id), false) => p.skip(ty)?,
                    ((badty, badid), true) => return ::std::result::Result::Err(::std::convert::From::from(
                        ::fbthrift::ApplicationException::new(
                            ::fbthrift::ApplicationExceptionErrorCode::ProtocolError,
                            format!(
                                "unwanted extra union {} field ty {:?} id {}",
                                "AdaptedTypesExn",
                                badty,
                                badid,
                            ),
                        )
                    )),
                }
                p.read_field_end()?;
            }
            p.read_struct_end()?;
            alt.ok_or_else(||
                ::fbthrift::ApplicationException::new(
                    ::fbthrift::ApplicationExceptionErrorCode::MissingResult,
                    format!("Empty union {}", "AdaptedTypesExn"),
                )
                .into(),
            )
        }
    }
}