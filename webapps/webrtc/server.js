const TrackerServer = require('bittorrent-tracker').Server;

const server = new TrackerServer({
    interval: 150000,
    udp: false,
    http: false,
    ws: true,
    stats: false,
    trustProxy: false,
    filter: (infoHash, params, cb) => {
        if (infoHash === '08ada5a7a6183aae1e09d831df6748d566095a10') {
            cb(null)
        } else {
            cb(new Error('test server, only sintel(08ada5a7a6183aae1e09d831df6748d566095a10) is allowed'))
        }
    }
})

server.on('error', err => console.log(err))
server.on('warning', err => console.log(err))
server.on('listening', () => {
    const wsAddr = server.ws.address()
    const wsHost = wsAddr.address !== '::' ? wsAddr.address : 'localhost'
    const wsPort = wsAddr.port
    console.log(`WebSocket tracker: ws://${wsHost}:${wsPort}`)
})
const port = process.env.PORT || 3001;
server.listen(port, "0.0.0.0")

server.on('start', (addr) => console.log('on-start:' + addr))
server.on('complete', (addr) => console.log('on-complete:' + addr))
server.on('update', (addr) => console.log('on-update:' + addr))
server.on('stop', (addr) => console.log('on-stopt:' + addr))
