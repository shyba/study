var WebTorrent = require('webtorrent')

var trackers = ['wss://webrtc-aquatic-demo.herokuapp.com']

var rtcConfig = {
  'iceServers': [
    {
      'urls': ['stun:stun.l.google.com:19305', 'stun:stun1.l.google.com:19305']
    }
  ]
}

var trackerOpts = {
  announce: trackers,
  rtcConfig: rtcConfig
}

var client = new WebTorrent({tracker: trackerOpts})


// Sintel, a free, Creative Commons movie
var torrentId = 'magnet:?xt=urn:btih:08ada5a7a6183aae1e09d831df6748d566095a10'

client.add(torrentId, {announce: trackers}, function (torrent) {
  // Torrents can contain many files. Let's use the .mp4 file
  var file = torrent.files.find(function (file) {
    return file.name.endsWith('.mp4')
  })

  // Display the file by adding it to the DOM.
  // Supports video, audio, image files, and more!
  file.appendTo('body')
})
