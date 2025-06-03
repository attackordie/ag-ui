# AG-UI WASM SDK Quick Reference

## Installation
```bash
# Build and copy package
git clone https://github.com/attackordie/ag-ui.git temp-ag-ui
cd temp-ag-ui/rust-sdk/ag-ui-wasm && wasm-pack build --target web
cp -r pkg/ ../../your-project/src/lib/ag-ui-wasm/
rm -rf temp-ag-ui
```

## Basic Usage
```typescript
import init, * as ag_ui from './lib/ag-ui-wasm/pkg/ag_ui_wasm.js';

// ALWAYS initialize first
await init();

// Create agent
const agent = new ag_ui.WebAgent('https://your-api.com/awp');

// Run agent
const result = await agent.run_agent_js({
  thread_id: 'thread-' + Date.now(),
  run_id: 'run-' + Date.now(),
  message: 'Hello!'
});
```

## React Hook
```typescript
import { useEffect, useState } from 'react';

export function useAgUi(endpoint: string) {
  const [agent, setAgent] = useState(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    async function load() {
      const ag_ui = await import('./lib/ag-ui-wasm/pkg/ag_ui_wasm.js');
      await ag_ui.default();
      setAgent(new ag_ui.WebAgent(endpoint));
      setIsLoading(false);
    }
    load();
  }, [endpoint]);

  return { agent, isLoading };
}
```

## Cloudflare Worker
```typescript
import * as ag_ui from './lib/ag-ui-wasm/pkg/ag_ui_wasm.js';

export default {
  async fetch(request: Request, env: Env): Promise<Response> {
    const agent = new ag_ui.WebAgent(env.AG_UI_ENDPOINT);
    const stream = await agent.run_agent_js({
      thread_id: crypto.randomUUID(),
      run_id: crypto.randomUUID(),
      message: await request.text()
    });
    
    return new Response(stream, {
      headers: { 'Content-Type': 'text/event-stream' }
    });
  }
};
```

## Stream Processing
```typescript
async function processStream(stream: ReadableStream) {
  const reader = stream.getReader();
  const decoder = new TextDecoder();
  
  try {
    while (true) {
      const { done, value } = await reader.read();
      if (done) break;
      
      const chunk = decoder.decode(value, { stream: true });
      const lines = chunk.split('\n');
      
      for (const line of lines) {
        if (line.startsWith('data: ')) {
          const data = line.slice(6);
          if (data === '[DONE]') return;
          
          const event = JSON.parse(data);
          console.log('Event:', event);
        }
      }
    }
  } finally {
    reader.releaseLock();
  }
}
```

## Essential Rules
1. **Always `await init()` first** before using any AG-UI functions
2. **Use `--target web`** when building with wasm-pack
3. **Handle streams properly** with getReader() and releaseLock()
4. **Validate endpoints** before creating agents
5. **Wrap in try-catch** for error handling

## Build Configs

### Vite
```typescript
export default defineConfig({
  optimizeDeps: { exclude: ['ag_ui_wasm'] },
  server: { fs: { allow: ['..', './src'] } }
});
```

### Next.js
```javascript
module.exports = {
  webpack: (config, { isServer }) => {
    if (!isServer) {
      config.experiments = { asyncWebAssembly: true };
    }
    return config;
  }
};
```

### TypeScript
```json
{
  "compilerOptions": {
    "target": "ES2020",
    "typeRoots": ["./src/lib/ag-ui-wasm/pkg"]
  }
}
```

## Error Handling
```typescript
try {
  const result = await agent.run_agent_js(input);
} catch (error) {
  if (error.message.includes('fetch')) {
    throw new Error('Network error');
  }
  throw new Error(`Agent error: ${error}`);
}
```

This is a Rust WASM SDK optimized for V8 isolates (browsers/Cloudflare Workers). It uses Web APIs (Fetch, Streams, SSE) instead of native Rust networking libraries. 