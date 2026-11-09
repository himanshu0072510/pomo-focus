/* tslint:disable */
/* eslint-disable */

export function best_hours(hour_counts: Uint32Array | number[]): Uint32Array;

export function challenge_progress(current: number, target: number): number;

export function compress_text(input: string): string;

export function compute_focus_score(total_poms: number, focus_minutes: number, streak: number, tasks_done: number): number;

export function decompress_text(input: string): string;

export function decrypt_text(ciphertext_b64: string, password: string): string;

export function dft_magnitude(samples: Float32Array | number[]): Float32Array;

export function encrypt_text(plaintext: string, password: string): string;

export function extract_keywords(text: string): string[];

export function generate_forest_seed(count: number): number;

export function productivity_insights(stats_json: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly best_hours: (a: number, b: number) => [number, number];
    readonly challenge_progress: (a: number, b: number) => number;
    readonly compress_text: (a: number, b: number) => [number, number];
    readonly compute_focus_score: (a: number, b: number, c: number, d: number) => number;
    readonly decompress_text: (a: number, b: number) => [number, number];
    readonly decrypt_text: (a: number, b: number, c: number, d: number) => [number, number];
    readonly dft_magnitude: (a: number, b: number) => [number, number];
    readonly encrypt_text: (a: number, b: number, c: number, d: number) => [number, number];
    readonly extract_keywords: (a: number, b: number) => [number, number];
    readonly generate_forest_seed: (a: number) => number;
    readonly productivity_insights: (a: number, b: number) => [number, number];
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_drop_slice: (a: number, b: number) => void;
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
