use std::fmt::Write;
use std::{env, fs};
use std::fs::FileType;
use std::iter::Map;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
struct CategoryInfo{
    name: String,
    description: String,
    system: String,
}

#[derive(Deserialize)]
struct ErrorInfo{
    name: String,
    message: String,
    short_description: String,
    long_description: String,
    short_solution: String,
    long_solution: String,
    support_link: String,
}

fn main(){
    println!("cargo:rerun-if-changed=./error-codes");

    let mut code = "pub fn get_error_code_and_category(category: u16, code: u16) -> (super::CategoryInfo, super::ErrorInfo) {\
                              let category_result = match category{\
                                ".to_owned();

    for category_dir in fs::read_dir("./error-codes/data").expect("unable to read dir"){
        let category_dir = category_dir.expect("unable to read category");

        let mut path = category_dir.path();
        path.push("en_US.json");

        let json_raw = fs::read_to_string(path).expect("unable to read category json");
        let category: Value = serde_json::from_str(&json_raw).expect("unable to parse json file");

        let Value::Object(categories) = category else {
            panic!("unable to parse category json");
        };

        let mut iter = categories.iter();

        let first_category = iter.next().unwrap();

        assert_eq!(iter.next(), None);

        let Value::Object(category_info) = first_category.1 else {
            panic!("unable to parse category json");
        };

        let Value::String(name) = &category_info["name"] else {
            panic!("unable to parse category json");
        };

        let Value::String(description) = &category_info["description"] else {
            panic!("unable to parse category json");
        };

        let Value::String(system) = &category_info["system"] else {
            panic!("unable to parse category json");
        };

        code.write_str(
            &format!("\
            {} => super::CategoryInfo{{\
                name: \"{}\",
                description: \"{}\",
                system: \"{}\",
            }},\
            ", first_category.0, name, description, system)
        ).expect("unable to write");

    }

    code.write_str("\
        _ => super::CategoryInfo::default()\
    };\
                            let error_result = match (category, code){\
                               \
    ").expect("unable to write");

    for category_dir in fs::read_dir("./error-codes/data").expect("unable to read dir"){
        let category_dir = category_dir.expect("unable to read category");

        for error_dir in fs::read_dir(category_dir.path()).expect("unable to read dir") {
            let error_dir = error_dir.expect("unable to read error");

            if !error_dir.file_type().unwrap().is_dir(){
                continue;
            }

            let mut path = error_dir.path();
            path.push("en_US.json");

            let json_raw = fs::read_to_string(path).expect("unable to read error json");
            let category: Value = serde_json::from_str(&json_raw).expect("unable to parse json file");

            let Value::Object(categories) = category else {
                panic!("unable to parse category json");
            };

            let mut iter = categories.iter();

            let first_category = iter.next().unwrap();

            assert_eq!(iter.next(), None);

            let Value::Object(category) = first_category.1 else {
                panic!("unable to parse category json");
            };

            let mut iter = category.iter();

            let first_error = iter.next().unwrap();

            assert_eq!(iter.next(), None);

            let Value::Object(error) = first_error.1 else {
                panic!("unable to parse category json");
            };

            if first_error.0.contains("X"){
                continue;
            }

            let category_num: u32 = first_category.0.parse().expect("unable to parse category");
            let error_num: u32 = first_error.0.parse().expect("unable to parse error");

            let Value::String(name) = &error["name"] else {
                panic!("unable to parse error json");
            };

            let name = name.replace("\"", "\\\"")
                .replace("\\n", "\n");

            let Value::String(message) = &error["message"] else {
                panic!("unable to parse error json");
            };

            let message = message.replace("\"", "\\\"")
                .replace("\\n", "\n");

            let Value::String(short_description) = &error["short_description"] else {
                panic!("unable to parse error json");
            };

            let short_description = short_description.replace("\"", "\\\"")
                .replace("\\n", "\n");

            let Value::String(long_description) = &error["long_description"] else {
                panic!("unable to parse error json");
            };

            let long_description = long_description.replace("\"", "\\\"")
                .replace("\\n", "\n");

            let Value::String(short_solution) = &error["short_solution"] else {
                panic!("unable to parse error json");
            };

            let short_solution = short_solution.replace("\"", "\\\"")
                .replace("\\n", "\n");

            let Value::String(long_solution) = &error["long_solution"] else {
                panic!("unable to parse error json");
            };

            let long_solution = long_solution.replace("\"", "\\\"")
                .replace("\\n", "\n");

            let Value::String(support_link) = &error["support_link"] else {
                panic!("unable to parse error json");
            };

            let support_link = support_link.replace("\"", "\\\"")
                .replace("\\n", "\n");

            code.write_str(
                &format!("\
            ({},{}) => super::ErrorInfo{{\
                name: \"{}\",
                message: \"{}\",
                short_description: \"{}\",
                long_description: \"{}\",
                short_solution: \"{}\",
                long_solution: \"{}\",
                support_link: \"{}\",
            }},\
            ", category_num, error_num, name, message, short_description, long_description, short_solution, long_solution, support_link)
            ).expect("unable to write");
        }
    }

    code.write_str("\
     _ => super::ErrorInfo::default()\
    };\
            (category_result, error_result)\
    }\
    ").expect("unable to write");
    let mut path = env::var("OUT_DIR").unwrap();
    path += "/errors.rs";
    fs::write(path, code).expect("unable to write generated code")
}