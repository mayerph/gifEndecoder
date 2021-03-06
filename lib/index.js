const gifendecoder = require('../native');
const resolve = require('path').resolve

function decodeGif(srcFile, dstDirectory, callback) {
    gifendecoder.decode(resolve(srcFile), resolve(dstDirectory), callback)
}

const encodeGif = (gifMeta, dstFile, infinite, speed, callback) => {
    // encode
    gifendecoder.encode(resolve(dstFile), gifMeta, infinite, speed, callback);  
}

function encodeWithUri(gifMeta, dstFile, infinite, speed) {
    return new Promise((resolve, reject) => {
        try {
            const result = gifendecoder.ecode_with_uri(resolve(dstFile), gifMeta, infinite, speed)
            resolve(result)
        } catch (err) {
            reject(err)
        }
    })
    
}

exports.encodeGif = encodeGif
exports.decodeGif = decodeGif
exports.encodeWithUri = encodeWithUri





