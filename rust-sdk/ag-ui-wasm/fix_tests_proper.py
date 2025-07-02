#!/usr/bin/env python3
import os
import re

def fix_message_struct(content):
    # Fix Message struct initializations
    pattern = r'(Message\s*\{[^}]+metadata:\s*[^,]+,)\s*(created_at:[^}]+\})'
    replacement = r'\1\n            name: None,\n            tool_calls: None,\n            function_call: None,\n            \2'
    content = re.sub(pattern, replacement, content, flags=re.MULTILINE | re.DOTALL)
    return content

def fix_runinput_struct(content):
    # Fix RunAgentInput struct initializations
    pattern = r'(RunAgentInput\s*\{[^}]+state:\s*[^,]+,)\s*(\})'
    replacement = r'\1\n        forwarded_props: None,\2'
    content = re.sub(pattern, replacement, content, flags=re.MULTILINE | re.DOTALL)
    return content

def fix_context_vec(content):
    # Fix context: Some(Context) to context: Some(vec![Context])
    content = re.sub(r'context: Some\(Context \{', r'context: Some(vec![Context {', content)
    content = re.sub(r'context: Some\(context\)', r'context: Some(vec![context])', content)
    # Fix the closing brackets
    content = re.sub(r'(context: Some\(vec\!\[Context \{[^}]+\})\)', r'\1])', content)
    return content

def fix_context_access(content):
    # Fix assert for context access
    content = re.sub(r'\.context\.as_ref\(\)\.unwrap\(\)\.user_id', 
                     r'.context.as_ref().unwrap()[0].user_id', content)
    return content

# Process all test files
test_dir = 'tests'
for filename in os.listdir(test_dir):
    if filename.endswith('.rs'):
        filepath = os.path.join(test_dir, filename)
        with open(filepath, 'r') as f:
            content = f.read()
        
        original = content
        content = fix_message_struct(content)
        content = fix_runinput_struct(content)
        content = fix_context_vec(content)
        content = fix_context_access(content)
        
        if content != original:
            with open(filepath, 'w') as f:
                f.write(content)
            print(f"Updated {filename}")

print("Test files updated!")