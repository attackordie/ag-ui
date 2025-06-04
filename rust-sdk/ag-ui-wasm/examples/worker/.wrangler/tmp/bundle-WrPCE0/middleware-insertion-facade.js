				import worker, * as OTHER_EXPORTS from "/home/brian/ag-ui/rust-sdk/ag-ui-wasm/examples/worker/pkg/ag_ui_worker_example.js";
				import * as __MIDDLEWARE_0__ from "/home/brian/.npm-global/lib/node_modules/wrangler/templates/middleware/middleware-ensure-req-body-drained.ts";
import * as __MIDDLEWARE_1__ from "/home/brian/.npm-global/lib/node_modules/wrangler/templates/middleware/middleware-miniflare3-json-error.ts";

				export * from "/home/brian/ag-ui/rust-sdk/ag-ui-wasm/examples/worker/pkg/ag_ui_worker_example.js";
				const MIDDLEWARE_TEST_INJECT = "__INJECT_FOR_TESTING_WRANGLER_MIDDLEWARE__";
				export const __INTERNAL_WRANGLER_MIDDLEWARE__ = [
					
					__MIDDLEWARE_0__.default,__MIDDLEWARE_1__.default
				]
				export default worker;