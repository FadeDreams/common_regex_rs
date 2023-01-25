extern crate regex;
use regex::Regex;


pub fn is_positive_int(s: &str) -> bool {
    let re = regex::Regex::new(r"^\d+$").unwrap();
    re.is_match(s)
}

pub fn is_int(s: &str) -> bool {
    s.parse::<i32>().is_ok()
}


pub fn is_decimal_num(s: &str) -> bool {
    let re = regex::Regex::new(r"^\d*(\.\d+)?$").unwrap();
    re.is_match(s)
}

pub fn is_num(s: &str) -> bool {
    let re = Regex::new(r"^-?\d*(\.\d+)?$").unwrap();
    return re.is_match(s);
}

pub fn is_alpha_numeric(s: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9]*$").unwrap();
    return re.is_match(s);
}

pub fn is_alpha_numeric_with_space(s: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9 ]*$").unwrap();
    return re.is_match(s);
}


pub fn is_email(email: &str) -> bool {
    let re = Regex::new(r"^([a-zA-Z0-9._-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,6})$").unwrap();
    return re.is_match(email);
}

pub fn is_good_password(password: &str) -> bool {
    return password.len() >= 8
        && Regex::new(r"[A-Z]").unwrap().is_match(password)
        && Regex::new(r"[a-z]").unwrap().is_match(password)
        && Regex::new(r"[0-9]").unwrap().is_match(password)
        && Regex::new(r"[^\w\s]").unwrap().is_match(password);
}

pub fn is_username(username: &str) -> bool {
    let re = Regex::new(r"^(?![-_]*$)[A-Za-z0-9][A-Za-z0-9-_]{3,20}[A-Za-z0-9]$").unwrap();
    return re.is_match(username);
}

//fn is_url(url: &str) -> bool {
    //// Use a regular expression to check if the string is a URL with the http or https protocol
    //let re = Regex::new(r#"(([\w]+:)?//)?(([\d\w]|%[a-fA-F\d]{2,2})+(:([\d\w]|%[a-fA-f\d]{2,2})+)?@)?([\d\w][-\d\w]{0,253}[\d\w]\.)+[\w]{2,4}(:[\d]+)?(/([-+_~.\d\w]|%[a-fA-f\d]{2,2})*)*(\?(&?([-+_~.\d\w]|%[a-fA-f\d]{2,2})=?)*)?(#([-+_~.\d\w]|%[a-fA-f\d]{2,2})*)?"#).unwrap();
    //re.is_match(url)
//}

pub fn is_url(url: &str) -> bool {
    let re = Regex::new(r"https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,4}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*)").unwrap();
    re.is_match(url)
}

pub fn is_ipv4(ip: &str) -> bool {
    let re = Regex::new(r"^((25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])\.){3}(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])$").unwrap();
    re.is_match(ip)
}


pub fn is_ipv6(ipv6: &str) -> bool {
    // Use a regular expression to check if the string is a URL with the http or https protocol
    let re = Regex::new(r"^([0-9A-Fa-f]{0,4}:){2,7}([0-9A-Fa-f]{1,4}$|((25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)(\.|$)){4})$").unwrap();
    re.is_match(ipv6)
}

pub fn is_date_yyyy_mm_dd(date: &str) -> bool {
    // Use a regular expression to check if the string is a URL with the http or https protocol
    let re = Regex::new(r"^([12]\d{3}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01]))$").unwrap();
    re.is_match(date)
}

pub fn is_date_dd_mm_yyyy(date: &str) -> bool {
    // Use a regular expression to check if the string is a URL with the http or https protocol
    let re = Regex::new(r"^((0[1-9]|[12]\d|3[01])-(0[1-9]|1[0-2])-[12]\d{3})$").unwrap();
    re.is_match(date)
}

pub fn is_time_hh_mm_12h(time: &str) -> bool {
    // Use a regular expression to check if the string is a time in the format hh:mm am/pm
    let re = Regex::new(r"((1[0-2]|0?[1-9]):([0-5][0-9])( ?([AaPp][Mm]))?)").unwrap();
    re.is_match(time)
}

pub fn is_time_hh_mm_24h(time: &str) -> bool {
    // Use a regular expression to check if the string is a time in the format hh:mm
    let re = Regex::new(r"^([0-9]|0[0-9]|1[0-9]|2[0-3]):[0-5][0-9]$").unwrap();
    re.is_match(time)
}

pub fn is_html_tag(html: &str) -> bool {
    // Use a regular expression to check if the string is an HTML tag
    let re = Regex::new(r#"<[/]?[a-z][^>]*>"#).unwrap();
    re.is_match(html)
}

pub fn is_slug(slug: &str) -> bool {
    // Use a regular expression to check if the string is a slug
    let re = Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap();
    re.is_match(slug)
}

pub fn is_tel(tel: &str) -> bool {
    // Use a regular expression to check if the string is a telephone number
    // let re = Regex::new(r"^(?:(?:\(?(?:00|\+)([1-4]\d\d|[1-9]\d?)\)?)?[\-\.\ \\\/]?)?((?:\(?\d{1,}\)?[\-\.\ \\\/]?){0,})(?:[\-\.\ \\\/]?(?:#|ext\.?|extension|x)[\-\.\ \\\/]?(\d+))?$").unwrap();
    let re = Regex::new(r"^(\+\d{1,2})?[-.\s]?(\d{3})[-.\s]?(\d{3})[-.\s]?(\d{4})\s*$").unwrap();
    // let re = Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap();
    re.is_match(tel)
}

