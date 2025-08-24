use strsim::levenshtein;

static STATUS_CODES: &[(usize, &str)] = &[
    (400, "BadRequest"),
    (401, "Unauthorized"),
    (402, "PaymentRequired"),
    (403, "Forbidden"),
    (404, "NotFound"),
    (405, "MethodNotAllowed"),
    (406, "NotAcceptable"),
    (407, "ProxyAuthenticationRequired"),
    (408, "RequestTimeout"),
    (409, "Conflict"),
    (410, "Gone"),
    (411, "LengthRequired"),
    (412, "PreconditionFailed"),
    (413, "PayloadTooLarge"),
    (414, "UriTooLong"),
    (415, "UnsupportedMediaType"),
    (416, "RangeNotSatisfiable"),
    (417, "ExpectationFailed"),
    (418, "ImATeapot"),
    (421, "MisdirectedRequest"),
    (422, "UnprocessableEntity"),
    (423, "Locked"),
    (424, "FailedDependency"),
    (425, "TooEarly"),
    (426, "UpgradeRequired"),
    (428, "PreconditionRequired"),
    (429, "TooManyRequests"),
    (431, "RequestHeaderFieldsTooLarge"),
    (451, "UnavailableForLegalReasons"),
    (500, "InternalServerError"),
    (501, "NotImplemented"),
    (502, "BadGateway"),
    (503, "ServiceUnavailable"),
    (504, "GatewayTimeout"),
    (505, "HttpVersionNotSupported"),
    (506, "VariantAlsoNegotiates"),
    (507, "InsufficientStorage"),
    (508, "LoopDetected"),
    (510, "NotExtended"),
    (511, "NetworkAuthenticationRequired"),
];

pub fn allowed_status_pairs() -> &'static [(usize, &'static str)] {
    STATUS_CODES
}

pub fn code_to_status(target_code: usize) -> Option<&'static str> {
    STATUS_CODES
        .iter()
        .find(|(supported_code, _)| &target_code == supported_code)
        .map(|(_, supported_status)| *supported_status)
}

pub fn is_status_supported(target_status: &str) -> bool {
    STATUS_CODES
        .iter()
        .any(|(_, supported_status)| &target_status == supported_status)
}

pub fn closest_status(target_status: &str) -> &str {
    STATUS_CODES
        .iter()
        .min_by_key(|(_, supported_status)| levenshtein(supported_status, target_status))
        .map_or_else(|| unreachable!(), |(_, supported_status)| *supported_status)
}
