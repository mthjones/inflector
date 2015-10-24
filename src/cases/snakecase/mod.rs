use std::ascii::*;
use regex::Regex;

use cases::classcase::is_class_case;
use cases::kebabcase::is_kebab_case;
use cases::camelcase::is_camel_case;
use cases::sentencecase::is_sentence_case;
use cases::titlecase::is_title_case;
use cases::lowercase::to_lower_case;

pub fn to_snake_case<'a>(non_snake_case_string: String) -> String {
    let mut snake_string: String = non_snake_case_string.clone();
    if is_camel_case(non_snake_case_string.clone()) {
        snake_string = to_snake_from_camel_or_class(non_snake_case_string.clone());
    }else if is_kebab_case(non_snake_case_string.clone()) {
        snake_string = to_snake_from_kebab(non_snake_case_string.clone());
    }else if is_sentence_case(non_snake_case_string.clone()) {
        snake_string = to_snake_from_sentence(non_snake_case_string.clone());
    }else if is_title_case(non_snake_case_string.clone()) {
        snake_string = to_snake_from_sentence(non_snake_case_string.clone());
    }else if is_class_case(non_snake_case_string.clone()) {
        snake_string = to_snake_from_camel_or_class(non_snake_case_string.clone());
    }
    return snake_string;
}
    fn to_snake_from_camel_or_class <'a>(non_snake_case_string: String) -> String {
        let re = Regex::new(r"(?P<a>[A-Z0-9][a-z]*)").unwrap();
        let result: String = re.replace_all(&non_snake_case_string.clone(), "_$a").to_string();
        return to_lower_case(result.trim_left_matches("_").to_string());
    }

    fn to_snake_from_kebab<'a>(non_snake_case_string: String) -> String {
        return to_lower_case(non_snake_case_string.replace("-", "_"));
    }

    fn to_snake_from_sentence<'a>(non_snake_case_string: String) -> String {
        return to_lower_case(non_snake_case_string.replace(" ", "_"));
    }

pub fn is_snake_case<'a>(test_string: String) -> bool{
    let snake_matcher = Regex::new(r"(?:[^-|^ ]?=^|[_])([a-z]+)").unwrap();
    let upcase_matcher = Regex::new(r"[A-Z]").unwrap();
    let mut is_snake_case = false;
    if snake_matcher.is_match(test_string.as_ref())
        && !upcase_matcher.is_match(test_string.as_ref()) {
            is_snake_case = true;
        }
    return is_snake_case;
}


#[cfg(test)]
mod snakecase_test;
