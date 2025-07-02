use std::fs;

fn main() {
    println!("Fixing test files...");
    
    // Fix comprehensive_types_test.rs
    fix_comprehensive_types_test();
    
    // Fix other test files
    fix_integration_test();
    fix_missing_python_coverage_test();
    fix_comprehensive_events_test();
    fix_typescript_inspired_tests();
    
    println!("All test files have been fixed!");
}

fn fix_comprehensive_types_test() {
    let content = fs::read_to_string("tests/comprehensive_types_test.rs").unwrap();
    
    // Already fixed the first issue, need to fix remaining ones
    let fixed = content
        // Fix developer role test
        .replace("(Role::Tool, \"tool\"),", "(Role::Tool, \"tool\"),\n        (Role::Developer, \"developer\"),")
        // Fix RunAgentInput test context field
        .replace("context: Some(context),", "context: Some(vec![context]),")
        .replace("context: Some(Context {", "context: Some(vec![Context {")
        .replace("            metadata: None,\n        }),", "            metadata: None,\n        }]),")
        // Fix assert
        .replace(".context.as_ref().unwrap().user_id", ".context.as_ref().unwrap()[0].user_id");
    
    fs::write("tests/comprehensive_types_test.rs", fixed).unwrap();
    println!("Fixed comprehensive_types_test.rs");
}

fn fix_integration_test() {
    let content = fs::read_to_string("tests/integration_test.rs").unwrap();
    
    // Remove misplaced fields from metadata blocks
    let lines: Vec<&str> = content.lines().collect();
    let mut fixed_lines = Vec::new();
    let mut skip_next = 0;
    
    for (i, line) in lines.iter().enumerate() {
        if skip_next > 0 {
            skip_next -= 1;
            continue;
        }
        
        // Check for misplaced fields after metadata
        if line.trim().starts_with("metadata:") && i + 1 < lines.len() {
            fixed_lines.push(line);
            
            // Check if next lines are the misplaced fields
            let mut j = i + 1;
            while j < lines.len() && (
                lines[j].trim() == "name: None," ||
                lines[j].trim() == "tool_calls: None," ||
                lines[j].trim() == "function_call: None,"
            ) {
                skip_next += 1;
                j += 1;
            }
        } else {
            fixed_lines.push(line);
        }
    }
    
    let mut fixed = fixed_lines.iter().map(|s| s.to_string()).collect::<Vec<String>>().join("\n");
    
    // Add missing fields to Message structs properly
    fixed = fixed.replace(
        "                tool_call_id: None,\n                metadata: None,\n                created_at:",
        "                name: None,\n                tool_call_id: None,\n                tool_calls: None,\n                function_call: None,\n                metadata: None,\n                created_at:"
    );
    
    // Fix RunAgentInput
    fixed = fixed.replace(
        "        state: Some(HashMap::new()),\n    };",
        "        state: Some(HashMap::new()),\n        forwarded_props: None,\n    };"
    );
    
    fs::write("tests/integration_test.rs", fixed).unwrap();
    println!("Fixed integration_test.rs");
}

fn fix_missing_python_coverage_test() {
    let content = fs::read_to_string("tests/missing_python_coverage_test.rs").unwrap();
    
    // Fix all Message structs with misplaced fields
    let mut fixed = content.to_string();
    
    // Pattern to fix metadata blocks with misplaced fields
    let lines: Vec<&str> = fixed.lines().collect();
    let mut new_lines = Vec::new();
    let mut i = 0;
    
    while i < lines.len() {
        let line = lines[i];
        
        if line.contains("metadata: Some({") && i + 1 < lines.len() {
            // Check if next lines contain misplaced fields
            let mut j = i + 1;
            let mut skip_fields = Vec::new();
            
            while j < lines.len() && (
                lines[j].trim() == "name: None," ||
                lines[j].trim() == "tool_calls: None," ||
                lines[j].trim() == "function_call: None,"
            ) {
                skip_fields.push(j);
                j += 1;
            }
            
            if !skip_fields.is_empty() {
                new_lines.push(line);
                // Skip the misplaced fields
                i = j;
                continue;
            }
        }
        
        new_lines.push(line);
        i += 1;
    }
    
    fixed = new_lines.join("\n");
    
    // Add missing fields to Message structs
    fixed = fixed.replace(
        "        tool_call_id: Some(\"call_456\".to_string()),\n        metadata: None,\n        created_at:",
        "        name: None,\n        tool_call_id: Some(\"call_456\".to_string()),\n        tool_calls: None,\n        function_call: None,\n        metadata: None,\n        created_at:"
    );
    
    fixed = fixed.replace(
        "        tool_call_id: None,\n        metadata: None,\n        created_at:",
        "        name: None,\n        tool_call_id: None,\n        tool_calls: None,\n        function_call: None,\n        metadata: None,\n        created_at:"
    );
    
    fixed = fixed.replace(
        "        tool_call_id: Some(\"call_789\".to_string()),\n        metadata: None,\n        created_at:",
        "        name: None,\n        tool_call_id: Some(\"call_789\".to_string()),\n        tool_calls: None,\n        function_call: None,\n        metadata: None,\n        created_at:"
    );
    
    fs::write("tests/missing_python_coverage_test.rs", fixed).unwrap();
    println!("Fixed missing_python_coverage_test.rs");
}

fn fix_comprehensive_events_test() {
    let content = fs::read_to_string("tests/comprehensive_events_test.rs").unwrap();
    
    // Fix Message structs
    let fixed = content.replace(
        "            tool_call_id: None,\n            metadata: None,\n            name: None,\n            tool_calls: None,\n            function_call: None,\n            created_at:",
        "            name: None,\n            tool_call_id: None,\n            tool_calls: None,\n            function_call: None,\n            metadata: None,\n            created_at:"
    );
    
    fs::write("tests/comprehensive_events_test.rs", fixed).unwrap();
    println!("Fixed comprehensive_events_test.rs");
}

fn fix_typescript_inspired_tests() {
    let content = fs::read_to_string("tests/typescript_inspired_tests.rs").unwrap();
    
    // Similar fixes as other files
    let mut fixed = content.to_string();
    
    // Fix metadata blocks with misplaced fields
    let lines: Vec<&str> = fixed.lines().collect();
    let mut new_lines = Vec::new();
    let mut i = 0;
    
    while i < lines.len() {
        let line = lines[i];
        
        if line.contains("metadata: Some({") && i + 1 < lines.len() {
            // Check if next lines contain misplaced fields
            let mut j = i + 1;
            
            while j < lines.len() && (
                lines[j].trim() == "name: None," ||
                lines[j].trim() == "tool_calls: None," ||
                lines[j].trim() == "function_call: None,"
            ) {
                j += 1;
            }
            
            if j > i + 1 {
                new_lines.push(line);
                // Skip the misplaced fields
                i = j;
                continue;
            }
        }
        
        new_lines.push(line);
        i += 1;
    }
    
    fixed = new_lines.join("\n");
    
    fs::write("tests/typescript_inspired_tests.rs", fixed).unwrap();
    println!("Fixed typescript_inspired_tests.rs");
}