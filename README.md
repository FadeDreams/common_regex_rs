### common_regex_rs

A Rust library that provides a collection of functions for validating common types of strings, such as emails, passwords, URLs, and more. It uses the regex crate to perform the regular expression matching.

#### Usage

To use the library, add common_regex_rs as a dependency in your Cargo.toml file and include the library in your project:

```
extern crate common_regex_rs;
use common_regex_rs::*;
```

Then, you can call the various functions provided by the library to check if a given string matches a certain pattern. For example, to check if a string is a valid email address:

```rust
let email = "example@email.com";
let is_valid = is_email(email);
```

Here is the list of all the functions that are provided by the library:

- 'is_positive_int' : check if the input string is a positive integer.
- 'is_int' : check if the input string is an integer.
- 'is_decimal_num' : check if the input string is decimal number.
- 'is_num' : check if the input string is a number.
- 'is_alpha_numeric' : check if the input string is alphanumeric.
- 'is_alpha_numeric_with_space' : check if the input string is alphanumeric with spaces.
- 'is_email' : check if the input string is a valid email address.
- 'is_good_password' : check if the input string is a good password with atleast 8 characters, one uppercase, one lowercase, one digit and one special character.
- 'is_username' : check if the input string is valid username with length between 4 and 20 characters and can contain alphanumeric characters, hyphens and underscores.
- 'is_url' : check if the input string is a URL with the http or https protocol.

#### Notes

- Please be careful when using regular expressions, as they can be a source of security vulnerabilities if not used properly.
- For more complex validation, it is recommended to use a dedicated library such as validator.
- The is_url function is commented out because it is not matching all valid URLs, if you need a function to check URLs, please use a library such as url or hyper.

#### License

This library is licensed under the MIT license. See the LICENSE file for details.
