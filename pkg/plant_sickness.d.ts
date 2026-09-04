/* tslint:disable */
/* eslint-disable */

/**
 * Chroma subsampling format
 */
export enum ChromaSampling {
    /**
     * Both vertically and horizontally subsampled.
     */
    Cs420 = 0,
    /**
     * Horizontally subsampled.
     */
    Cs422 = 1,
    /**
     * Not subsampled.
     */
    Cs444 = 2,
    /**
     * Monochrome.
     */
    Cs400 = 3,
}

/**
 * Filter options.
 */
export class FilterOptions {
    free(): void;
    [Symbol.dispose](): void;
    constructor(contrast: number, brightness: number, blur_sigma: number, grayscale: boolean);
    blur_sigma: number;
    brightness: number;
    contrast: number;
    grayscale: boolean;
}

export class ProcessedResult {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    readonly normalized_tensor: Float32Array;
    readonly rgba_bytes: Uint8Array;
}

/**
 * Pipeline:
 * Decoding -> EXIF Correction -> Filtering -> Resizing (224x224) -> Leaf Segmentation (HSV) -> Normalization
 */
export function process_image_full(image_bytes: Uint8Array, filters?: FilterOptions | null): ProcessedResult;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_filteroptions_free: (a: number, b: number) => void;
    readonly __wbg_get_filteroptions_blur_sigma: (a: number) => number;
    readonly __wbg_get_filteroptions_brightness: (a: number) => number;
    readonly __wbg_get_filteroptions_contrast: (a: number) => number;
    readonly __wbg_get_filteroptions_grayscale: (a: number) => number;
    readonly __wbg_processedresult_free: (a: number, b: number) => void;
    readonly __wbg_set_filteroptions_blur_sigma: (a: number, b: number) => void;
    readonly __wbg_set_filteroptions_brightness: (a: number, b: number) => void;
    readonly __wbg_set_filteroptions_contrast: (a: number, b: number) => void;
    readonly __wbg_set_filteroptions_grayscale: (a: number, b: number) => void;
    readonly filteroptions_new: (a: number, b: number, c: number, d: number) => number;
    readonly process_image_full: (a: number, b: number, c: number) => [number, number, number];
    readonly processedresult_normalized_tensor: (a: number) => [number, number];
    readonly processedresult_rgba_bytes: (a: number) => [number, number];
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
