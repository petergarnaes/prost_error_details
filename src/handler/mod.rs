use crate::error_detail;
use prost_types::Any;
use prost::{Message, DecodeError};

pub enum ErrorDetailKind {
    BadRequest(error_detail::BadRequest),
    DebugInfo(error_detail::DebugInfo),
    ErrorInfo(error_detail::ErrorInfo),
    Help(error_detail::Help),
    LocalizedMessage(error_detail::LocalizedMessage),
    PreconditionFailure(error_detail::PreconditionFailure),
    QuotaFailure(error_detail::QuotaFailure),
    RequestInfo(error_detail::RequestInfo),
    ResourceInfo(error_detail::ResourceInfo),
    #[non_exhaustive]
    RetryInfo(error_detail::RetryInfo),
}

macro_rules! create_type_url {
    ($ident:ident) => {
        stringify!(type.googleapis.com/google.rpc.$ident)
    }
}

macro_rules! build_match_decode_case {
    [$any_var:ident, $($ident:ident),*] => {
        match $any_var.type_url.as_str() {
            $(
                create_type_url!($ident) => {
                    match error_detail::$ident::decode($any_var.value.as_slice()) {
                        Ok(message) => Ok(ErrorDetailKind::$ident(message)),
                        Err(decode_error) => Err(decode_error),
                    }
                },
            )*
            _ => Err(DecodeError::new("Unknown error detail message")),
        }
    }
}

pub fn decode_error_detail(any: &Any) -> Result<ErrorDetailKind, DecodeError> {
    build_match_decode_case![
        any,
        BadRequest,
        DebugInfo,
        ErrorInfo,
        Help,
        LocalizedMessage,
        PreconditionFailure,
        QuotaFailure,
        RequestInfo,
        ResourceInfo,
        RetryInfo
    ]
}
