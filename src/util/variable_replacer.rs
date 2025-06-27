use std::collections::HashMap;
use std::process::exit;
use unicode_segmentation::UnicodeSegmentation;
use colorize;
use colorize::AnsiColor;
use crate::util::emoji_to_string::VARIABLE_SELECTOR_EMOJI;
use crate::util::variable::Variable;

pub fn replace_variables_in_emoji_strings(string: String, variables: HashMap<String, Variable>) -> String {
    let mut output = String::new();
    let mut current_var_name = String::new();
    let mut in_variable = false;
    let uni_seperated_str: Vec<&str> = UnicodeSegmentation::graphemes(string.as_str(), true).collect();
    for char in uni_seperated_str{
        if char == VARIABLE_SELECTOR_EMOJI{
            if in_variable {
                eprintln!("{}", "Can't use variable selector emoji (💰) while still in a variable".red().to_string());
                exit(102);
            }
            in_variable = true;
            continue;
        }
        if in_variable {
            current_var_name += char;
            if let Some(var) = variables.get(&current_var_name) {
                in_variable = false;
                output.push_str(&var.value);
                current_var_name = String::new();
            }
            continue;
        }
        output = output + char;
    }

    output
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::util::variable::Variable;
    use crate::util::variable::VariableKind::String;
    use crate::util::variable_replacer::replace_variables_in_emoji_strings;

    #[test]
    fn test_replace_variables_in_emoji_strings() {
        let mut variables = HashMap::new();
        variables.insert("😀".to_string(), Variable::new(String, 0, true, "🔧⚙️".to_string()));
        variables.insert("💎🐾".to_string(), Variable::new(String, 0, true, "🐕🦝🌵".to_string()));
        let str = replace_variables_in_emoji_strings("🦫💰😀😁💰💎🐾🦫".to_string(), variables.clone());
        assert_eq!(str, "🦫🔧⚙️😁🐕🦝🌵🦫");
    }
}