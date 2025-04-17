use std::env::VarError;

pub fn replace_environment(str: &str) -> String {
    let mut s_builder: Vec<char> = Vec::new();
    let mut possible_env: Vec<char> = Vec::new();
    let mut in_var = false;
    for c in str.chars() {
        if in_var {
            if !c.is_alphanumeric() && c != '_' {
                match environment_value(&possible_env) {
                    Ok(Some(val)) => {
                        for c in val.chars() {
                            s_builder.push(c);
                        }
                    },
                    Ok(None) => {
                        s_builder.push('$');
                        let _ = &possible_env.into_iter().for_each(|c| {
                            s_builder.push(c);
                        });
                    },
                    // TODO: Handle error
                    Err(e) => panic!("{}", e),
                }
                s_builder.push(c);
                possible_env = Vec::new();
                in_var = false;
            } else {
                possible_env.push(c);
            }
        } else if c == '$' {
            in_var = true;
        } else {
            s_builder.push(c);
        }
    }

    s_builder.into_iter().collect()
}

fn environment_value(key: &Vec<char>) -> Result<Option<String>, VarError>{
    let key_s: String = key.into_iter().collect();

    match std::env::var(key_s) {
        Ok(val) => Ok(Some(val)),
        Err(VarError::NotPresent) => Ok(None),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::environment::replace_environment;

    #[test]
    fn no_env_set() {
        assert_eq!("ignore me", replace_environment(&"ignore me".to_owned()));
    }

    #[test]
    fn replaces_environment_variables() {
        unsafe {
            std::env::set_var("FOOBAR", "foobar");
        }

        assert_eq!("replace foobar please", replace_environment(&"replace $FOOBAR please".to_owned()));
    }

    #[test]
    fn supports_variables_with_underscore() {
        unsafe {
            std::env::set_var("FOO_BAR", "foobar");
        }

        assert_eq!("replace foobar please", replace_environment(&"replace $FOO_BAR please".to_owned()));
    }

    #[test]
    fn replaces_multiple_variables() {
        unsafe {
            std::env::set_var("FOOBAR", "foobar");
            std::env::set_var("FOO_BAR", "foobar");
            std::env::set_var("FOOBAZ", "foobaz");
        }

        assert_eq!("replace foobar foobaz foobar please",
            replace_environment(&"replace $FOOBAR $FOOBAZ $FOO_BAR please".to_owned()));
    }

    #[test]
    fn ignores_unset_variables() {
        assert_eq!("replace $FOOBARMISSING please", replace_environment(&"replace $FOOBARMISSING please".to_owned()));
    }

    /*
     * TODO:
     * 1. Removes quotes if wrapped in quotes
     * 2. Allows different ways to reference environment variables
     */
}