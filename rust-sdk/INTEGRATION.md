# AG-UI Rust SDK Integration Quick Start

> **TL;DR**: Clone → Build → Copy `pkg/` → Import → Use

## 🚀 5-Minute Setup

### 1. Get the Package

```bash
git clone https://github.com/attackordie/ag-ui.git
cd ag-ui/rust-sdk/ag-ui-wasm
wasm-pack build --target web
```

### 2. Copy to Your Project

```bash
# Copy the built package to your project
cp -r pkg/ /path/to/your/project/ag-ui-wasm/
```

### 3. Import and Use

```javascript
import init, * as ag_ui from './ag-ui-wasm/ag_ui_wasm.js';

await init(); // Initialize WASM
const agent = new ag_ui.WebAgent('https://your-api.com/awp');
```

## 📋 Project-Specific Instructions

### React/Next.js
```bash
# In your React project root
cp -r /path/to/ag-ui/rust-sdk/ag-ui-wasm/pkg/ ./src/lib/ag-ui-wasm/
```

```tsx
// components/AgentClient.tsx
import { useEffect, useState } from 'react';

export function AgentClient() {
  const [agUi, setAgUi] = useState(null);

  useEffect(() => {
    async function loadSdk() {
      const sdk = await import('../lib/ag-ui-wasm/ag_ui_wasm.js');
      await sdk.default(); // Initialize
      setAgUi(sdk);
    }
    loadSdk();
  }, []);

  return agUi ? <div>AG-UI Ready!</div> : <div>Loading...</div>;
}
```

### Cloudflare Workers
```bash
# In your worker project
cp -r /path/to/ag-ui/rust-sdk/ag-ui-wasm/pkg/ ./node_modules/ag-ui-wasm/
```

```javascript
// worker.js
import * as ag_ui from 'ag-ui-wasm';

export default {
  async fetch(request, env) {
    const agent = new ag_ui.WebAgent(env.AG_UI_ENDPOINT);
    const result = await agent.run_agent_js({
      thread_id: 'thread-1',
      run_id: 'run-1'
    });
    return new Response(JSON.stringify(result));
  }
};
```

### Vite
```bash
# Copy to public directory for easy access
cp -r /path/to/ag-ui/rust-sdk/ag-ui-wasm/pkg/ ./public/ag-ui-wasm/
```

```typescript
// src/main.ts
import init, * as ag_ui from '/ag-ui-wasm/ag_ui_wasm.js';

async function main() {
  await init();
  const agent = new ag_ui.WebAgent('https://api.example.com/awp');
  // Use agent...
}

main();
```

### Vanilla HTML
```html
<!DOCTYPE html>
<html>
<head>
  <script type="module">
    import init, * as ag_ui from './ag-ui-wasm/ag_ui_wasm.js';
    
    window.addEventListener('load', async () => {
      await init();
      window.agUi = ag_ui; // Make globally available
      console.log('AG-UI SDK loaded!');
    });
  </script>
</head>
<body>
  <script>
    // Use window.agUi after page loads
    setTimeout(() => {
      const agent = new window.agUi.WebAgent('https://api.example.com/awp');
    }, 1000);
  </script>
</body>
</html>
```

## 🔧 Different Build Targets

### For Browsers/Workers (Default)
```bash
wasm-pack build --target web
```

### For Node.js
```bash
wasm-pack build --target nodejs
```

### For Bundlers (Webpack, etc.)
```bash
wasm-pack build --target bundler
```

## 📦 What You Get

After building, `pkg/` contains:

- **`ag_ui_wasm.js`** - Main JavaScript interface
- **`ag_ui_wasm_bg.wasm`** - WebAssembly binary
- **`ag_ui_wasm.d.ts`** - TypeScript definitions
- **`package.json`** - NPM metadata

## ⚡ Basic Usage Patterns

### Simple Agent Run
```javascript
const agent = new ag_ui.WebAgent('https://api.example.com/awp');
const result = await agent.run_agent_js({
  thread_id: 'thread-1',
  run_id: 'run-1'
});
```

### With Error Handling
```javascript
try {
  const agent = new ag_ui.WebAgent('https://api.example.com/awp');
  const result = await agent.run_agent_js(input);
  console.log('Success:', result);
} catch (error) {
  console.error('Failed:', error);
}
```

### Streaming Events
```javascript
// TODO: Add streaming example when implemented
```

## 🚨 Common Issues

### ❌ "Module not found"
**Solution**: Make sure you've run `wasm-pack build --target web`

### ❌ TypeScript errors
**Solution**: Ensure `ag_ui_wasm.d.ts` is in your TypeScript path

### ❌ CORS errors in browser
**Solution**: Serve files via HTTP server, not `file://` protocol

### ❌ Cloudflare Worker deployment fails
**Solution**: Use `--target web`, not `--target nodejs`

## 📚 Next Steps

1. **Read the full docs**: [`rust-sdk/ag-ui-wasm/README.md`](./ag-ui-wasm/README.md)
2. **Check examples**: [`rust-sdk/ag-ui-wasm/examples/`](./ag-ui-wasm/examples/)
3. **API reference**: [`rust-sdk/ag-ui-wasm/README.md#api-reference`](./ag-ui-wasm/README.md#api-reference)

## 🆘 Need Help?

- Check the [main README](./README.md) for detailed instructions
- Look at the [worker example](./ag-ui-wasm/examples/worker/) for a complete implementation
- Review the [API documentation](./ag-ui-wasm/README.md#api-reference) for all available methods 