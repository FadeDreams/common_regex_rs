extern crate common_regex_rs;

use common_regex_rs::{  
is_positive_int, is_int, is_decimal_num,
is_num, is_alpha_numeric,
is_alpha_numeric_with_space, is_email,
is_good_password, is_username,
is_url, is_ipv4,
is_ipv6, is_date_yyyy_mm_dd,
is_date_dd_mm_yyyy, is_time_hh_mm_12h,
is_time_hh_mm_24h, is_html_tag,
is_slug, is_tel

};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_positive_int() {
        assert!(is_positive_int("42"));
        assert!(is_positive_int("1"));
        assert!(!is_positive_int("-42"));
        assert!(!is_positive_int("42.5"));
        assert!(!is_positive_int("foo"));
        assert!(!is_positive_int(""));
    }

    #[test]
    fn test_is_int() {
        assert!(is_int("42"));
        assert!(is_int("-42"));
        assert!(!is_int("42.5"));
        assert!(!is_int("foo"));
        assert!(!is_int(""));
    }

    #[test]
    fn test_is_decimal_num() {
        assert!(is_decimal_num("123"));
        assert!(is_decimal_num("123.456"));
        assert!(is_decimal_num(".123"));
        assert!(is_decimal_num("0.123"));
        assert!(!is_decimal_num("abc"));
        assert!(!is_decimal_num("123abc"));
        assert!(!is_decimal_num("-123"));
        assert!(!is_decimal_num("123."));
    }

    #[test]
    fn test_is_num() {
        assert!(is_num("123"));
        assert!(is_num("-123"));
        assert!(is_num("123.456"));
        assert!(is_num("-123.456"));
        assert!(!is_num("abc"));
        assert!(!is_num("123abc"));
    }

    #[test]
    fn test_is_alpha_numeric() {
        assert!(is_alpha_numeric("abc123"));
        assert!(is_alpha_numeric("ABC123"));
        assert!(is_alpha_numeric("123"));
        assert!(!is_alpha_numeric("abc123!"));
        assert!(!is_alpha_numeric("abc 123"));
    }

    #[test]
    fn test_is_alpha_numeric_with_space() {
        assert!(is_alpha_numeric_with_space("abc 123"));
        assert!(is_alpha_numeric_with_space("ABC 123"));
        assert!(is_alpha_numeric_with_space("123"));
        assert!(is_alpha_numeric_with_space("abc123"));
        assert!(!is_alpha_numeric_with_space("abc123!"));
        assert!(!is_alpha_numeric_with_space("abc 123!"));
    }

    #[test]
    fn test_is_email() {
        assert!(is_email("user@google.com"));
        assert!(is_email("user.name@google.com"));
        assert!(is_email("user_name@google.com"));
        assert!(!is_email("user@google"));
        assert!(!is_email("@google.com"));
        assert!(!is_email("user@"));
    }

    #[test]
    fn test_is_good_password() {
        assert!(is_good_password("Abc123!@"));
        assert!(is_good_password("Abc123!@A"));
        assert!(!is_good_password("abc123"));
        assert!(!is_good_password("ABC123"));
        assert!(!is_good_password("ABC123!"));
        assert!(!is_good_password("abc123!"));
    }

    fn test_is_username() {
        assert!(is_username("abc123"));
        assert!(is_username("abc-123"));
        assert!(is_username("abc_123"));
        assert!(is_username("abc-123_456"));
        assert!(!is_username("abc"));
        assert!(!is_username("123"));
        assert!(!is_username("abc123!"));
        assert!(!is_username("---"));
        assert!(!is_username("___"));
    }

    #[test]
    fn test_is_url() {
        assert!(is_url("http://www.google.com"));
        assert!(is_url("https://www.google.com"));
        assert!(!is_url("ftp://www.google.com"));
        assert!(!is_url("www.google.com"));
    }

    #[test]
    fn test_is_ipv4() {
        assert!(is_ipv4("192.168.0.1"));
        assert!(is_ipv4("10.0.0.1"));
    }

    #[test]
    fn test_is_ipv6() {
        assert!(is_ipv6("2001:0db8:85a3:0000:0000:8a2e:0370:7334"));
        assert!(is_ipv6("2001:0db8:85a3::8a2e:0370:7334"));
        assert!(!is_ipv6("2001:0db8:85a3:0:0:8a2e:0370:7334:0"));
        assert!(!is_ipv6("localhost"));
        assert!(!is_ipv6("127.0.0.1"));
    }

    #[test]
    fn test_is_date_yyyy_mm_dd() {
        assert!(is_date_yyyy_mm_dd("2022-12-24"));
        assert!(is_date_yyyy_mm_dd("2022-01-01"));
        assert!(!is_date_yyyy_mm_dd("2022-12-32"));
        assert!(!is_date_yyyy_mm_dd("2022-13-01"));
    }

    #[test]
    fn test_is_date_dd_mm_yyyy() {
        assert!(is_date_dd_mm_yyyy("24-12-2022"));
        assert!(is_date_dd_mm_yyyy("01-01-2022"));
        assert!(!is_date_dd_mm_yyyy("32-12-2022"));
        assert!(!is_date_dd_mm_yyyy("01-13-2022"));
        assert!(!is_date_dd_mm_yyyy("2022-12-24"));
    }

    #[test]
    fn test_is_time_hh_mm_12h() {
        assert!(is_time_hh_mm_12h("12:00 PM"));
        assert!(is_time_hh_mm_12h("12:34 PM"));
        assert!(is_time_hh_mm_12h("1:00 AM"));
        assert!(is_time_hh_mm_12h("11:59 AM"));
        assert!(is_time_hh_mm_12h("12:00 AM"));
        assert!(is_time_hh_mm_12h("12:34 AM"));
        assert!(is_time_hh_mm_12h("1:00 PM"));
        assert!(is_time_hh_mm_12h("11:59 PM"));

    }

    #[test]
    fn test_is_time_hh_mm_24h() {
        assert!(is_time_hh_mm_24h("00:00"));
        assert!(is_time_hh_mm_24h("00:01"));
        assert!(is_time_hh_mm_24h("23:59"));
        assert!(is_time_hh_mm_24h("12:34"));

        assert!(!is_time_hh_mm_24h("24:00"));
        assert!(!is_time_hh_mm_24h("12:00 AM"));
        assert!(!is_time_hh_mm_24h("12:60"));
        assert!(!is_time_hh_mm_24h("abc"));
    }

    #[test]
    fn test_is_html_tag() {
        assert!(is_html_tag("<div>"));
        assert!(is_html_tag("</div>"));
        assert!(is_html_tag("<p>"));
        assert!(is_html_tag("</p>"));
        assert!(is_html_tag("<a href='http://example.com'>"));
        assert!(is_html_tag("<img src='image.jpg'>"));

    }


    #[test]
    fn test_is_slug() {
        assert!(is_slug("abc"));
        assert!(is_slug("abc-def"));
        assert!(is_slug("abc-def-ghi"));
        assert!(is_slug("a-b-c-d-e-f"));
        assert!(is_slug("123"));
        assert!(is_slug("123-456"));
        assert!(is_slug("123-456-789"));
        assert!(is_slug("1-2-3-4-5-6"));
        assert!(is_slug("abc123"));
        assert!(is_slug("abc123-def456"));
        assert!(is_slug("abc123-def456-ghi789"));
        assert!(is_slug("a-b-c123-d-e-f456"));

        assert!(!is_slug("a bc"));
        assert!(!is_slug("abc def"));
        assert!(!is_slug("abc-def ghi"));
        assert!(!is_slug("a-b-c-d-e-f "));
        assert!(!is_slug(" abc"));
        assert!(!is_slug("abc-def "));
        assert!(!is_slug("abc-def-ghi "));
        assert!(!is_slug(" a-b-c-d-e-f"));
        assert!(!is_slug(""));

    }

    #[test]
    fn test_is_tel() {
        assert!(is_tel("123-456-7890"));
        assert!(is_tel("123 456 7890"));
        assert!(is_tel("123 456 7890"));
        assert!(is_tel("123.456.7890"));
        assert!(is_tel("+1 123 456 7890"));
    }




}
