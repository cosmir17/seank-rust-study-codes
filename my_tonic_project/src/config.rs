#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(dead_code)]

use std::borrow::Cow;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct Config {
    pub endpoint: Cow<'static, str>,
    pub token: Cow<'static, str>,
}

pub const CONFIG: Config = Config {
    endpoint: Cow::Borrowed("http://[::1]:50051"),
    token: Cow::Borrowed("Bearer MYTOKEN"),
};
