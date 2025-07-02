use std::fs;
use std::path::Path;

fn main() {
    let test_dir = Path::new("tests");
    
    // Get all .rs files in tests directory
    let entries = fs::read_dir(test_dir).expect("Failed to read test directory");
    
    for entry in entries {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            let content = fs::read_to_string(&path).expect("Failed to read file");
            let fixed_content = fix_content(&content);
            
            if content != fixed_content {
                fs::write(&path, fixed_content).expect("Failed to write file");
                println!("Fixed: {}", path.display());
            }
        }
    }
    
    println!("All test files have been fixed!");
}

fn fix_content(content: &str) -> String {
    let mut result = content.to_string();
    
    // Fix malformed Message struct initialization
    // Pattern: metadata field followed by misplaced name/tool_calls/function_call fields
    let pattern = r"metadata: (Some\([^}]+\})\),\s*name: None,\s*tool_calls: None,\s*function_call: None,";
    let replacement = "name: None,\n        tool_call_id: None,\n        tool_calls: None,\n        function_call: None,\n        metadata: $1),";
    result = regex::Regex::new(pattern).unwrap()
        .replace_all(&result, replacement)
        .to_string();
    
    // Fix Message struct where fields are inside metadata block
    let lines: Vec<&str> = result.lines().collect();
    let mut fixed_lines = Vec::new();
    let mut i = 0;
    
    while i < lines.len() {
        let line = lines[i];
        
        // Check for malformed metadata block with name/tool_calls/function_call inside
        if line.trim().starts_with("metadata: Some({") {
            fixed_lines.push(line);
            i += 1;
            
            // Skip the misplaced fields
            while i < lines.len() {
                let inner_line = lines[i];
                if inner_line.trim() == "name: None," ||
                   inner_line.trim() == "tool_calls: None," ||
                   inner_line.trim() == "function_call: None," {
                    i += 1;
                    continue;
                }
                fixed_lines.push(inner_line);
                if inner_line.trim().starts_with("}),") {
                    break;
                }
                i += 1;
            }
        } else {
            fixed_lines.push(line);
        }
        i += 1;
    }
    
    result = fixed_lines.join("\n");
    
    // Fix context field to be Vec<Context>
    result = result.replace("context: Some(Context {", "context: Some(vec![Context {");
    result = result.replace("context: Some(context)", "context: Some(vec![context])");
    
    // Fix closing bracket for vec![Context {...}]
    let pattern = r"(context: Some\(vec!\[Context \{[^}]+\})\)";
    let replacement = "$1])";
    result = regex::Regex::new(pattern).unwrap()
        .replace_all(&result, replacement)
        .to_string();
    
    // Fix context access in assertions
    result = result.replace(".context.as_ref().unwrap().user_id", ".context.as_ref().unwrap()[0].user_id");
    
    // Fix RunAgentInput missing forwarded_props
    let pattern = r"(RunAgentInput \{[^}]+state: [^,]+),\s*\}";
    let replacement = "$1,\n        forwarded_props: None,\n    }";
    result = regex::Regex::new(pattern).unwrap()
        .replace_all(&result, replacement)
        .to_string();
    
    result
}