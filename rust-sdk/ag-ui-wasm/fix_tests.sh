#!/bin/bash

# Fix Message struct initializations in all test files
find tests -name "*.rs" -type f -exec sed -i '
/Message {/,/}/ {
    /metadata:/ {
        a\            name: None,
        a\            tool_calls: None,
        a\            function_call: None,
    }
}' {} \;

# Fix RunAgentInput struct initializations
find tests -name "*.rs" -type f -exec sed -i '
/RunAgentInput {/,/}/ {
    /state:/ {
        a\        forwarded_props: None,
    }
}' {} \;

# Fix context: Some(Context) to context: Some(vec![Context])
find tests -name "*.rs" -type f -exec sed -i '
s/context: Some(Context {/context: Some(vec![Context {/g
s/context: Some(context)/context: Some(vec![context])/g
' {} \;

# Fix assert for context access
find tests -name "*.rs" -type f -exec sed -i '
s/\.context\.as_ref()\.unwrap()\.user_id/.context.as_ref().unwrap()[0].user_id/g
' {} \;

echo "Test files updated!"