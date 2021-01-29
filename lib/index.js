const gifendecoder = require('../native');
const resolve = require('path').resolve

function decodeGif(srcFile, dstDirectory) {
    const result = gifendecoder.decode(resolve(srcFile), resolve(dstDirectory))
    return result
}

function encodeGif(gifMeta, dstFile, infinite, speed) {
    const result = gifendecoder.encode(resolve(dstFile), gifMeta, infinite, speed)
    return result
}

function encodeWithUri(gifMeta, dstFile, infinite, speed) {
    const result = gifendecoder.ecode_with_uri(resolve(dstFile), gifMeta, infinite, speed)
    return result
}

exports.encodeGif = encodeGif
exports.decodeGif = decodeGif
exports.encodeWithUri = encodeWithUri





