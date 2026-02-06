use crate::http::request::Request;

pub struct Case {
    pub name: &'static str,
    pub req: Request,
    pub expected_status: (u16, u16),
    pub expected_timeout: bool,
    pub expected_body: Option<&'static str>,
}