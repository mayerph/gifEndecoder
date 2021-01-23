var gifendecoder = require('../native');


//const gifTemplate = gifendecoder.decode("./storage/gif/templates/abjk31ssft/pinguin.gif", "./storage/gif/templates/abjk31ssft/frames")
//console.log(gifendecoder.encode("./storage/gif/memes/a.gif", gifTemplate, true))

function decodeGif(srcFile, dstDirectory) {
    console.log("the srcFile is", srcFile)
    console.log("the dstFile is", dstDirectory)
    const result = gifendecoder.decode(srcFile, dstDirectory)
    console.log("the result is", JSON.stringify(result))
    return result
}

function encodeGif(gifMeta, dstFile, infinite) {
    const result = gifendecoder.encode(dstFile, gifMeta, infinite)
    return result
}

exports.encodeGif = encodeGif
exports.decodeGif = decodeGif
// const template = decodeGif("./storage/gif/templates/abjk31ssft/pinguin.gif", "./storage/gif/templates/abjk31ssft/frames")
// encodeGif(template, "./storage/gif/memes/a.gif", true)




