export interface GifMeta {
    file: string,
    frames: {
        delay: {
            numerator: number,
            denominator: number
        },
        file: string,
        left: number,
        top: number
    }[],
}