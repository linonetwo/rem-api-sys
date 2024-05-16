use crate::build_utils::{config::HandlerConfigs, format_name::get_full_name_of_entity};
use clang::*;

use super::ParameterFlavor;

macro_rules! console_debug {
  ($($tokens: tt)*) => {
      println!("cargo:warning={}", format!($($tokens)*))
  }
}

pub fn get_pointer_parameter(
    name: &str,
    entity_type: &Type,
    configs: &mut HandlerConfigs,
) -> String {
    let pointee_type = entity_type.get_pointee_type().unwrap();
    let flavor = &configs.parameter_flavor;
    // console_debug!("get_pointer_parameter {:?} {:?}", pointee_type, flavor,);
    match pointee_type.get_kind() {
        TypeKind::CharS => match flavor {
            // 这个是char*, register_front() 会有这样的参数
            ParameterFlavor::MethodCallParam => format!("{}.into_raw()", name),
            ParameterFlavor::Rust | ParameterFlavor::RustStruct => "std::ffi::CString".to_string(),
            ParameterFlavor::SpiFn => "*const std::os::raw::c_char".to_string(),
            ParameterFlavor::UnsafeCheck => "/* No checking 2 */".to_string(),
            ParameterFlavor::None => "/* char* */".to_string(),
        },
        TypeKind::UChar => match flavor {
            ParameterFlavor::MethodCallParam => format!("{} as *mut ::std::os::raw::c_uchar", name),
            ParameterFlavor::Rust | ParameterFlavor::RustStruct => "u8".to_string(),
            ParameterFlavor::SpiFn => "*const std::os::raw::c_uchar".to_string(),
            ParameterFlavor::UnsafeCheck => "/* No checking 3 */".to_string(),
            ParameterFlavor::None => "/* unsigned char* */".to_string(),
        },
        TypeKind::Pointer => {
            let inner_type = pointee_type.get_pointee_type().unwrap();
            let inner_type_kind = inner_type.get_kind();
            match inner_type.get_kind() {
              TypeKind::CharS => match flavor {
                  ParameterFlavor::MethodCallParam => format!("{}.as_ptr() as *const *const i8", name),
                  ParameterFlavor::RustStruct => format!("{}", name),
                  ParameterFlavor::Rust => "Vec<std::ffi::CString>".to_string(),
                  ParameterFlavor::SpiFn => ".iter().map(|s| s.as_ptr()).collect::<Vec<_>>().as_ptr() as *const *const i8".to_string(),
                  ParameterFlavor::UnsafeCheck => format!("/* No checking 4 {:?} */", inner_type_kind),
                  ParameterFlavor::None => "/* char** */".to_string(),
              },
              _ => panic!("Unhandled pointer to pointer type"),
          }
        }
        TypeKind::Record => {
            let decl = pointee_type.get_declaration().unwrap();
            let entity_name = get_full_name_of_entity(&decl);
            // console_debug!("TypeKind::Record {:?} {:?} {:?}", decl, entity_name, flavor,);
            match flavor {
                ParameterFlavor::MethodCallParam => format!("&mut *{}", name),
                ParameterFlavor::Rust => {
                    configs.life_time_on_children = true;
                    if configs.prefer_pointer {
                        format!("*{}mut {}", configs.life_time, entity_name)
                    } else {
                        format!("&{}mut {}", configs.life_time, entity_name)
                    }
                }
                ParameterFlavor::RustStruct => {
                    configs.life_time_on_children = true;
                    format!("&{}{}", configs.life_time, entity_name)
                }
                ParameterFlavor::SpiFn => format!("{}", entity_name),
                ParameterFlavor::UnsafeCheck => format!("/* No checking 5 {} */", entity_name),
                ParameterFlavor::None => format!("/* {} */", entity_name),
            }
        }
        TypeKind::Elaborated => {
            let pointee_name = pointee_type.get_display_name();
            match flavor {
                ParameterFlavor::MethodCallParam => {
                    if entity_type.is_const_qualified() {
                        format!("{}", name)
                    } else {
                        format!("{} as *mut", name)
                    }
                }
                ParameterFlavor::Rust | ParameterFlavor::RustStruct => {
                    if entity_type.is_const_qualified() {
                        format!("&{}", pointee_name)
                    } else {
                        format!("&mut {}", pointee_name)
                    }
                }
                ParameterFlavor::SpiFn => format!("&{}", pointee_name),
                ParameterFlavor::UnsafeCheck => format!("/* No checking 6 {:?} */", pointee_name),

                ParameterFlavor::None => format!("/* {} */", pointee_name),
            }
        }
        _ => {
            if let Some(decl) = pointee_type.get_declaration() {
                let entity_name = get_full_name_of_entity(&decl);
                match flavor {
                    ParameterFlavor::MethodCallParam => {
                        if configs.prefer_pointer {
                            format!("&mut *{}", name)
                        } else {
                            format!("{}", name)
                        }
                    }
                    ParameterFlavor::Rust | ParameterFlavor::RustStruct => {
                        configs.life_time_on_children = true;
                        if configs.prefer_pointer {
                            format!("*mut {}{}", configs.life_time, entity_name)
                        } else {
                            format!("&{}{}", configs.life_time, entity_name)
                        }
                    }
                    ParameterFlavor::SpiFn => format!("&{}", entity_name),
                    ParameterFlavor::UnsafeCheck => {
                        format!("/* No checking 7 {:?} */", entity_name)
                    }
                    ParameterFlavor::None => format!("/* {} */", entity_name),
                }
            } else {
                panic!("Unhandled pointer type: {:?}", pointee_type.get_kind());
            }
        }
    }
}
