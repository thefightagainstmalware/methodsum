/* tslint:disable */
/* eslint-disable */
/**
*/
export class MethodSum {
  free(): void;
/**
* @param {Uint8Array} insns
*/
  constructor(insns: Uint8Array);
/**
* @param {string} a
* @param {string} b
* @returns {number}
*/
  static compare(a: string, b: string): number;
/**
* @returns {string}
*/
  get_hash(): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_methodsum_free: (a: number) => void;
  readonly methodsum_init: (a: number, b: number) => number;
  readonly methodsum_compare: (a: number, b: number, c: number, d: number) => number;
  readonly methodsum_get_hash: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
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
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
