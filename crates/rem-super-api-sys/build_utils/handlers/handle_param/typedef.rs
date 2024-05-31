use crate::build_utils::{config::HandlerConfigs, format_name::get_full_name_of_entity};
use clang::*;

use super::{get_pointer_parameter, ParameterFlavor};

pub fn get_typedef_parameter(
    name: &str,
    entity_type: &Type,
    configs: &mut HandlerConfigs,
) -> String {
    let underlying_type = entity_type
        .get_declaration()
        .unwrap()
        .get_typedef_underlying_type()
        .unwrap();
    let flavor = &configs.parameter_flavor;
    let underlying_type_kind = underlying_type.get_kind();
    match underlying_type_kind {
        TypeKind::CharS => match flavor {
            ParameterFlavor::MethodCallParam => "*const std::os::raw::c_char".to_string(),
            ParameterFlavor::Rust | ParameterFlavor::RustStruct => {
                "std::os::raw::c_char".to_string()
            }
            ParameterFlavor::SpiFn => "as *const std::os::raw::c_char".to_string(),
            ParameterFlavor::UnsafeCheck => {
                format!("/* No checking 8 {:?} */", underlying_type_kind)
            }
            ParameterFlavor::None => "/* c_char */".to_string(),
        },
        TypeKind::Pointer => get_pointer_parameter(&name, &underlying_type, configs), // Delegate to the pointer handler
        TypeKind::Int => match flavor {
            ParameterFlavor::MethodCallParam
            | ParameterFlavor::Rust
            | ParameterFlavor::RustStruct => "i32".to_string(),
            ParameterFlavor::SpiFn => "as i32".to_string(),
            ParameterFlavor::UnsafeCheck => {
                format!("/* No checking 9 {:?} */", underlying_type_kind)
            }
            ParameterFlavor::None => "/* int */".to_string(),
        },
        TypeKind::Bool => match flavor {
            ParameterFlavor::MethodCallParam
            | ParameterFlavor::Rust
            | ParameterFlavor::RustStruct => "bool".to_string(),
            ParameterFlavor::SpiFn => "as bool".to_string(),
            ParameterFlavor::UnsafeCheck => {
                format!("/* No checking 10 {:?} */", underlying_type_kind)
            }
            ParameterFlavor::None => "/* bool */".to_string(),
        },
        TypeKind::Elaborated | TypeKind::Record => {
            // Handle user-defined types, structs, and unions
            let decl = underlying_type.get_declaration().unwrap();
            let type_name = get_full_name_of_entity(&decl);
            match flavor {
                ParameterFlavor::MethodCallParam => format!("*mut {}", type_name),
                ParameterFlavor::Rust | ParameterFlavor::RustStruct => type_name,
                ParameterFlavor::SpiFn => format!("as *mut {}", type_name),
                ParameterFlavor::UnsafeCheck => {
                    format!("/* No checking 11 {:?} */", underlying_type_kind)
                }
                ParameterFlavor::None => format!("/* {} */", type_name),
            }
        }
        TypeKind::ConstantArray => {
            let array_type = underlying_type.get_element_type().unwrap();
            let size = underlying_type.get_size().unwrap();
            match array_type.get_kind() {
                TypeKind::CharS => match flavor {
                    ParameterFlavor::MethodCallParam
                    | ParameterFlavor::Rust
                    | ParameterFlavor::RustStruct => format!("[std::os::raw::c_char; {}]", size),
                    ParameterFlavor::SpiFn => format!("as *const [std::os::raw::c_char; {}]", size),
                    ParameterFlavor::UnsafeCheck => {
                        format!("/* No checking 12 {:?} */", underlying_type_kind)
                    }
                    ParameterFlavor::None => "/* char array */".to_string(),
                },
                TypeKind::Int => match flavor {
                    ParameterFlavor::MethodCallParam
                    | ParameterFlavor::Rust
                    | ParameterFlavor::RustStruct => format!("[i32; {}]", size),
                    ParameterFlavor::SpiFn => format!("as *const [i32; {}]", size),
                    ParameterFlavor::UnsafeCheck => {
                        format!("/* No checking 13 {:?} */", underlying_type_kind)
                    }
                    ParameterFlavor::None => "/* int array */".to_string(),
                },
                // Add other type cases as needed
                _ => panic!(
                    "Unhandled constant array element type: {:?}",
                    array_type.get_kind()
                ),
            }
        }
        TypeKind::LongLong => match flavor {
            ParameterFlavor::MethodCallParam
            | ParameterFlavor::Rust
            | ParameterFlavor::RustStruct => {
                if underlying_type.is_const_qualified() {
                    "const i64".to_string() // or "const u64" if it's unsigned
                } else {
                    "i64".to_string() // or "u64" if it's unsigned
                }
            }
            ParameterFlavor::SpiFn => "as i64".to_string(), // or "as u64" if it's unsigned
            ParameterFlavor::UnsafeCheck => {
                format!("/* No checking 14 {:?} */", underlying_type_kind)
            }
            ParameterFlavor::None => "/* i64 */".to_string(), // or "/* u64 */" if it's unsigned
        },
        TypeKind::Double => match flavor {
            ParameterFlavor::MethodCallParam
            | ParameterFlavor::Rust
            | ParameterFlavor::RustStruct => "f64".to_string(),
            ParameterFlavor::SpiFn => "as f64".to_string(),
            ParameterFlavor::UnsafeCheck => {
                format!("/* No checking 15 {:?} */", underlying_type_kind)
            }
            ParameterFlavor::None => "/* double */".to_string(),
        },
        TypeKind::Float => match flavor {
            ParameterFlavor::MethodCallParam
            | ParameterFlavor::Rust
            | ParameterFlavor::RustStruct => "f32".to_string(),
            ParameterFlavor::SpiFn => "as f32".to_string(),
            ParameterFlavor::UnsafeCheck => {
                format!("/* No checking 21 {:?} */", underlying_type_kind)
            }
            ParameterFlavor::None => "/* float */".to_string(),
        },
        TypeKind::LongDouble => match flavor {
            ParameterFlavor::MethodCallParam
            | ParameterFlavor::Rust
            | ParameterFlavor::RustStruct => "f64".to_string(), // or another appropriate type depending on the platform
            ParameterFlavor::SpiFn => "as f64".to_string(),
            ParameterFlavor::UnsafeCheck => {
                format!("/* No checking 23 {:?} */", underlying_type_kind)
            }
            ParameterFlavor::None => "/* long double */".to_string(),
        },
        TypeKind::UInt => match flavor {
            ParameterFlavor::MethodCallParam
            | ParameterFlavor::Rust
            | ParameterFlavor::RustStruct => "u32".to_string(),
            ParameterFlavor::SpiFn => "as u32".to_string(),
            ParameterFlavor::UnsafeCheck => {
                format!("/* No checking 9 {:?} */", underlying_type_kind)
            }
            ParameterFlavor::None => "/* unsigned int */".to_string(),
        },
        TypeKind::Short => match flavor {
            ParameterFlavor::MethodCallParam
            | ParameterFlavor::Rust
            | ParameterFlavor::RustStruct => "i16".to_string(),
            ParameterFlavor::SpiFn => "as i16".to_string(),
            ParameterFlavor::UnsafeCheck => {
                format!("/* No checking 16 {:?} */", underlying_type_kind)
            }
            ParameterFlavor::None => "/* short */".to_string(),
        },
        TypeKind::UShort => match flavor {
            ParameterFlavor::MethodCallParam
            | ParameterFlavor::Rust
            | ParameterFlavor::RustStruct => "u16".to_string(),
            ParameterFlavor::SpiFn => "as u16".to_string(),
            ParameterFlavor::UnsafeCheck => {
                format!("/* No checking 8 {:?} */", underlying_type_kind)
            }
            ParameterFlavor::None => "/* unsigned short */".to_string(),
        },
        TypeKind::ULongLong => match flavor {
            ParameterFlavor::MethodCallParam
            | ParameterFlavor::Rust
            | ParameterFlavor::RustStruct => "u64".to_string(),
            ParameterFlavor::SpiFn => "as u64".to_string(),
            ParameterFlavor::UnsafeCheck => {
                format!("/* No checking 11 {:?} */", underlying_type_kind)
            }
            ParameterFlavor::None => "/* unsigned long long */".to_string(),
        },
        TypeKind::WChar => match flavor {
            ParameterFlavor::MethodCallParam
            | ParameterFlavor::Rust
            | ParameterFlavor::RustStruct => "u16".to_string(),  // or "i32" depending on platform specifics
            ParameterFlavor::SpiFn => "as u16".to_string(),
            ParameterFlavor::UnsafeCheck => {
                format!("/* No checking 15 {:?} */", underlying_type_kind)
            }
            ParameterFlavor::None => "/* wchar_t */".to_string(),
        },
        TypeKind::SChar => match flavor {
            ParameterFlavor::MethodCallParam
            | ParameterFlavor::Rust
            | ParameterFlavor::RustStruct => "i8".to_string(),
            ParameterFlavor::SpiFn => "as i8".to_string(),
            ParameterFlavor::UnsafeCheck => {
                format!("/* No checking 14 {:?} */", underlying_type_kind)
            }
            ParameterFlavor::None => "/* signed char */".to_string(),
        },
        TypeKind::UChar => match flavor {
            ParameterFlavor::MethodCallParam
            | ParameterFlavor::Rust
            | ParameterFlavor::RustStruct => "u8".to_string(),
            ParameterFlavor::SpiFn => "as u8".to_string(),
            ParameterFlavor::UnsafeCheck => {
                format!("/* No checking 5 {:?} */", underlying_type_kind)
            }
            ParameterFlavor::None => "/* unsigned char */".to_string(),
        },        
        _ => panic!("Unhandled typedef type: {:?}", underlying_type.get_kind()),
    }
}
