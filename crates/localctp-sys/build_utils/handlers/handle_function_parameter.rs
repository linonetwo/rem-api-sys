use crate::build_utils::{
    config::HandlerConfigs, format_name::get_full_name_of_entity, handle_param::{param_flavor::ParameterFlavor, pointer::{ get_pointer_parameter}, typedef::get_typedef_parameter}, handle_record::RecordFlavor, Handler, HandlerMap
};
use clang::*;
use inflector::Inflector;

lazy_static! {
    static ref INDENT: String = "    ".to_string();
}

macro_rules! console_debug {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

pub fn insert_function_parameter_handlers(handlers: &mut HandlerMap) {
    let parameter_types_to_handle = [
        TypeKind::Bool,
        TypeKind::Double,
        TypeKind::Elaborated,
        TypeKind::IncompleteArray,
        TypeKind::Int,
        TypeKind::Pointer,
        TypeKind::Typedef,
        TypeKind::UInt,
        TypeKind::ULongLong,
        TypeKind::Enum,
    ];
    for type_kind in parameter_types_to_handle {
        handlers.insert(
            type_kind,
            Handler::FunctionPrototype(Box::new(handle_function_parameter)),
        );
    }
}
pub fn handle_function_parameter(
    entity: &Entity,
    handlers: &HandlerMap,
    configs: &mut HandlerConfigs,
) -> Vec<String> {
    let entity_type = entity.get_type().unwrap();
    let entity_name = if entity.get_name().unwrap() == "type" {
        "type_".to_string() // Rename "type" to "type_"
    } else {
        Inflector::to_snake_case(&entity.get_name().unwrap())
    };
    console_debug!(
        "handle_function_parameter {:?} {:?}",
        entity_name,
        entity_type.get_kind()
    );

    let parameter_str = match entity_type.get_kind() {
        TypeKind::Pointer => {
            let parameter = get_pointer_parameter(&entity_name, &entity_type, configs);
            match &configs.parameter_flavor {
                ParameterFlavor::MethodCallParam => parameter,
                _ => format_parameter(&entity_name, &parameter, &configs.parameter_flavor),
            }
        }
        TypeKind::Typedef => {
            let parameter = get_typedef_parameter(&entity_name, &entity_type, configs);
            match &configs.parameter_flavor {
                ParameterFlavor::MethodCallParam => parameter,
                _ => format_parameter(&entity_name, &parameter, &configs.parameter_flavor),
            }
        }
        TypeKind::UInt => match &configs.parameter_flavor {
            ParameterFlavor::UnsafeCheck => "".to_string(),
            _ => format_parameter(&entity_name, "std::os::raw::c_uint", &configs.parameter_flavor),
        },
        TypeKind::Double => match &configs.parameter_flavor {
            ParameterFlavor::UnsafeCheck => "".to_string(),
            _ => format_parameter(&entity_name, "f64", &configs.parameter_flavor),
        },
        TypeKind::ULongLong => match &configs.parameter_flavor {
            ParameterFlavor::UnsafeCheck => "".to_string(),
            _ => format_parameter(&entity_name, "u64", &configs.parameter_flavor),
        },
        TypeKind::Int => match &configs.parameter_flavor {
            ParameterFlavor::UnsafeCheck => "".to_string(),
            _ => format_parameter(&entity_name, "std::os::raw::c_int", &configs.parameter_flavor),
        },
        TypeKind::Bool => match &configs.parameter_flavor {
            ParameterFlavor::UnsafeCheck => "".to_string(),
            _ => format_parameter(&entity_name, "bool", &configs.parameter_flavor),
        },
        TypeKind::Enum => match &configs.parameter_flavor {
            ParameterFlavor::UnsafeCheck => "".to_string(),
            _ => {
                let d = entity_type.get_declaration().unwrap();
                let rust_type = get_full_name_of_entity(&d);
                format_parameter(&entity_name, &rust_type, &configs.parameter_flavor)
            }
        },
        TypeKind::IncompleteArray => match &configs.parameter_flavor {
            ParameterFlavor::UnsafeCheck => "".to_string(),
            ParameterFlavor::MethodCallParam => match &configs.record_flavor {
                RecordFlavor::SPI => format!("{}", &entity_name),
                _ => format!("{}.iter().map(|cs| cs.as_ptr()).collect::<Vec<_>>().as_mut_ptr() as *mut *mut i8", &entity_name),
            },
            _ => format_parameter(&entity_name, "Vec<std::ffi::CString>", &configs.parameter_flavor),
        },        
        _ => {
            println!("handle_function_parameter not handling {:?}", entity_type);
            panic!("");
        }
    };

    let is_only_child = configs.num_parent_children_same_handler == 1;
    let is_first_child = configs.index == 0;
    let is_last_child = configs.index == configs.num_parent_children_same_handler - 1;
    if is_only_child {
        match configs.parameter_flavor {
            ParameterFlavor::SpiFn => {
                vec![parameter_str, ",\n".to_string()]
            }
            _ => vec![parameter_str],
        }
    } else {
        match configs.parameter_flavor {
            ParameterFlavor::None | ParameterFlavor::UnsafeCheck => vec!["".to_string(), parameter_str],
            ParameterFlavor::RustStruct | ParameterFlavor::SpiFn => {
                vec![parameter_str, ",\n".to_string()]
            }
            _ => {
                if is_first_child {
                    vec![parameter_str]
                } else {
                    vec![", ".to_string(), parameter_str]
                }
            }
        }
    }
}

fn format_parameter(name: &str, parameter: &str, flavor: &ParameterFlavor) -> String {
    match flavor {
        ParameterFlavor::MethodCallParam => format!("{name}"),
        ParameterFlavor::Rust => format!("{name}: {parameter}"),
        ParameterFlavor::SpiFn => {
            format!("{indent}{indent}{indent}{name}: {name}", indent = *INDENT)
        }
        ParameterFlavor::RustStruct => {
            format!("{indent}pub {name}: {parameter}", indent = *INDENT)
        }
        ParameterFlavor::UnsafeCheck => format!("{indent}{indent}assert!(!{name}.is_null());", indent = *INDENT),
        ParameterFlavor::None => format!("/* Param: {name} */"),
    }
}
