use std::collections::HashMap;

use crate::helpers::status_codes::{allowed_status_pairs, closest_status, is_status_supported};

#[test]
pub fn allowed_status_pairs_getter() {
    fn find_status(pairs: &'static [(usize, &'static str)], code: usize) -> Option<&'static str> {
        pairs
            .iter()
            .find(|(c, _)| c == &code)
            .map(|(_, s)| *s)
    }

    let pairs = allowed_status_pairs();

    let allowed = HashMap::from(const {[
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
    ]});

    // check allowed codes
    for (code, name) in &allowed {
        assert_eq!(find_status(pairs, *code), Some(*name));
    }

    for code in 100..=599 {
        if !allowed.contains_key(&code) {
            assert_eq!(find_status(pairs, code), None);
        }
    }
}

#[test]
pub fn supported_status_codes() {
    const STATUS_TEST: [(&'static str, bool); 62] = [
        ("Continue", false),
        ("SwitchingProtocols", false),
        ("Processing", false),
        ("EarlyHints", false),
        ("OK", false),
        ("Created", false),
        ("Accepted", false),
        ("NonAuthoritativeInformation", false),
        ("NoContent", false),
        ("ResetContent", false),
        ("PartialContent", false),
        ("MultiStatus", false),
        ("AlreadyReported", false),
        ("IMUsed", false),
        ("MultipleChoices", false),
        ("MovedPermanently", false),
        ("Found", false),
        ("SeeOther", false),
        ("NotModified", false),
        ("UseProxy", false),
        ("TemporaryRedirect", false),
        ("PermanentRedirect", false),
        ("BadRequest", true),
        ("Unauthorized", true),
        ("PaymentRequired", true),
        ("Forbidden", true),
        ("NotFound", true),
        ("MethodNotAllowed", true),
        ("NotAcceptable", true),
        ("ProxyAuthenticationRequired", true),
        ("RequestTimeout", true),
        ("Conflict", true),
        ("Gone", true),
        ("LengthRequired", true),
        ("PreconditionFailed", true),
        ("PayloadTooLarge", true),
        ("UriTooLong", true),
        ("UnsupportedMediaType", true),
        ("RangeNotSatisfiable", true),
        ("ExpectationFailed", true),
        ("ImATeapot", true),
        ("MisdirectedRequest", true),
        ("UnprocessableEntity", true),
        ("Locked", true),
        ("FailedDependency", true),
        ("TooEarly", true),
        ("UpgradeRequired", true),
        ("PreconditionRequired", true),
        ("TooManyRequests", true),
        ("RequestHeaderFieldsTooLarge", true),
        ("UnavailableForLegalReasons", true),
        ("InternalServerError", true),
        ("NotImplemented", true),
        ("BadGateway", true),
        ("ServiceUnavailable", true),
        ("GatewayTimeout", true),
        ("HttpVersionNotSupported", true),
        ("VariantAlsoNegotiates", true),
        ("InsufficientStorage", true),
        ("LoopDetected", true),
        ("NotExtended", true),
        ("NetworkAuthenticationRequired", true),
    ];

    for (status, allowed) in STATUS_TEST.iter() {
        assert_eq!(is_status_supported(status), *allowed, "{status} {allowed}");
    }
}

#[test]
pub fn closest_status_codes() {
    static SLIGHT_ERRORS: [(&'static str, &'static str); 39] = [
        ("BadRequet", "BadRequest"),
        ("Unathorized", "Unauthorized"),
        ("PaymntRequired", "PaymentRequired"),
        ("Frobidden", "Forbidden"),
        ("NotFoud", "NotFound"),
        ("MethdNotAllowed", "MethodNotAllowed"),
        ("NotAceptable", "NotAcceptable"),
        ("ProxyAuthenticatonRequired", "ProxyAuthenticationRequired"),
        ("ReqestTimeout", "RequestTimeout"),
        ("Confict", "Conflict"),
        ("Goen", "Gone"),
        ("LenghtRequired", "LengthRequired"),
        ("PrecondtionFailed", "PreconditionFailed"),
        ("PayloadToLarge", "PayloadTooLarge"),
        ("UrlToLong", "UriTooLong"),
        ("UnsuppportedMediaType", "UnsupportedMediaType"),
        ("RangeNotSatisfable", "RangeNotSatisfiable"),
        ("ExpectionFailed", "ExpectationFailed"),
        ("ImATpot", "ImATeapot"),
        ("MisdirectedRequet", "MisdirectedRequest"),
        ("UnprocesableEntity", "UnprocessableEntity"),
        ("Lokced", "Locked"),
        ("FailedDependancy", "FailedDependency"),
        ("UpgradeRequiered", "UpgradeRequired"),
        ("PrecondtionRequired", "PreconditionRequired"),
        ("TooManyRequsts", "TooManyRequests"),
        ("RequestHeaderFieldsTooLarg", "RequestHeaderFieldsTooLarge"),
        ("UnavailableForLegalReasns", "UnavailableForLegalReasons"),
        ("InternalServerErro", "InternalServerError"),
        ("NotImplemeted", "NotImplemented"),
        ("BadGatewy", "BadGateway"),
        ("ServiceUnavaliable", "ServiceUnavailable"),
        ("GatewayTimeot", "GatewayTimeout"),
        ("HttpVersionNotSupportd", "HttpVersionNotSupported"),
        ("VariantAlsoNegociates", "VariantAlsoNegotiates"),
        ("InsufficientStorge", "InsufficientStorage"),
        ("LoopDetcted", "LoopDetected"),
        ("NotExtendd", "NotExtended"),
        ("NetworkAuthenticationRequiered", "NetworkAuthenticationRequired"),
    ];

    for (wrong, correct) in SLIGHT_ERRORS {
        assert_eq!(closest_status(wrong), correct)
    }
}
