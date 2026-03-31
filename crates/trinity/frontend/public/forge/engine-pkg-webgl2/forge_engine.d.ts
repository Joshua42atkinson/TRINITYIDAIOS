/* tslint:disable */
/* eslint-disable */

/**
 * Handle a command from JavaScript.
 * Accepts a command string and a JsValue payload, parses them,
 * and dispatches to the appropriate handler in core.
 */
export function handle_command(command: string, payload: any): any;

/**
 * Handle a batch of commands from JavaScript.
 *
 * Accepts a JsValue that deserialises to a JSON array of `{ command, payload? }` objects.
 * Returns a JsValue array of `CommandResponse` objects in the same order.
 *
 * This avoids the per-call JS↔WASM boundary overhead when dispatching many commands
 * in a single frame (e.g., during AI compound actions or scene import).
 */
export function handle_command_batch(batch: any): any;

/**
 * Initialize the Forge engine and attach to a canvas element.
 * This function is idempotent - subsequent calls are no-ops.
 */
export function init_engine(canvas_id: string): void;

/**
 * Send an event to the JavaScript frontend.
 * This is how the engine communicates back to React.
 */
export function set_event_callback(callback: Function): void;

/**
 * Set the callback function for initialization events.
 * Call this before init_engine to receive lifecycle events.
 */
export function set_init_callback(callback: Function): void;

/**
 * Update engine state from JSON scene graph (legacy API).
 */
export function update_scene(scene_json: string): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly handle_command: (a: number, b: number, c: any) => [number, number, number];
    readonly handle_command_batch: (a: any) => [number, number, number];
    readonly init_engine: (a: number, b: number) => [number, number];
    readonly set_event_callback: (a: any) => void;
    readonly set_init_callback: (a: any) => void;
    readonly update_scene: (a: number, b: number) => [number, number];
    readonly wasm_bindgen__closure__destroy__h19c28c7a51ccd65d: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__h803e0cbf57792659: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__hfe6d060b4a8690b7: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__h3772bb38311f9e91: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__h80749410d3a7f6ff: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__h16086f4e2486ebd9: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__h0f16ca21869b72a5: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__h88b00fe1c6a311eb: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__hd7e1ead558a98375: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__h81a7aa3a5601115f: (a: number, b: number) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h62909b5096aa33aa: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__hdfd59933e0de8bca: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h6da439a52ca57c1f: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h90df69bd15bd541a: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h75208787e0087664: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__ha79a732d7dbfeecb: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h536585999a485636: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h98a7a280c7cfb7ba: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__hcfb9558147691898: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h6cf682f3b1296ded: (a: number, b: number) => number;
    readonly wasm_bindgen__convert__closures_____invoke__hd256af75774dece4: (a: number, b: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
