import { GifMeta } from "./gifMeta.interface"

/**
 * decodes gif and writes down single frames
 * @param {string} srcFile the path to the gif file
 * @param {string} dstDirectory the location where the raw data should be stored
 * @returns {GifMeta} meta information of the decoded gif file
 */
export declare function decodeGif(srcFile: string, dstDirectory: string): Promise<GifMeta>

/**
 * 
 * @param {GifMeta} gifMeta  meta information of the gif file
 * @param {string} dstFile path to the gif file  
 * @param {string} infinite loop information of the singles frames
 * @returns {string} the path to the generated gif file
 */
export declare function encodeGif(gifMeta: GifMeta, dstFile: string, infinite: boolean, speed: number): Promise<string>


/**
 * 
 * @param {GifMeta} gifMeta  meta information of the gif file
 * @param {string} dstFile path to the gif file  
 * @param {string} infinite loop information of the singles frames
 * @returns {string} the path to the generated gif file
 */
export declare function encodeWithUri(gifMeta: GifMeta, dstFile: string, infinite: boolean, speed: number): Promise<string>


