/* tslint:disable */
/* eslint-disable */
/**
* @param {string} name
*/
export function greet(name: string): void;
/**
* @param {Uint8Array} _array
* @returns {string}
*/
export function grayscale(_array: Uint8Array): string;
/**
*/
export class Pipe {
  free(): void;
/**
*/
  constructor();
/**
* @param {string} t
* @param {any} value
*/
  add_feature(t: string, value: any): void;
/**
* @param {number} index
*/
  del_feature(index: number): void;
/**
* @param {number} index
* @param {any} param
*/
  set_feature(index: number, param: any): void;
/**
* @returns {number}
*/
  feature_len(): number;
/**
* @param {Uint8Array} arr
* @param {string} from_suffix
* @param {string} to_suffix
* @returns {string}
*/
  render(arr: Uint8Array, from_suffix: string, to_suffix: string): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly greet: (a: number, b: number) => void;
  readonly grayscale: (a: number, b: number, c: number) => void;
  readonly __wbg_pipe_free: (a: number) => void;
  readonly pipe_new: () => number;
  readonly pipe_add_feature: (a: number, b: number, c: number, d: number) => void;
  readonly pipe_del_feature: (a: number, b: number) => void;
  readonly pipe_set_feature: (a: number, b: number, c: number) => void;
  readonly pipe_feature_len: (a: number) => number;
  readonly pipe_render: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;