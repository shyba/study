# HDLBits

https://hdlbits.01xz.net/

Great website to learn Verilog. This folder contains all my answers. Please don't copy them and try to make it yourself, but if you feel stuck try opening issues and I can see how I can help.


### Extracting all submissions after finishing

on the [profile stats page](https://hdlbits.01xz.net/wiki/Special:VlgStats) run on Firefox Console:
```javascript
(function(console){

console.save = function(data, filename){

    if(!data) {
        console.error('Console.save: No data')
        return;
    }

    if(!filename) filename = 'console.json'

    if(typeof data === "object"){
        data = JSON.stringify(data, undefined, 4)
    }

    var blob = new Blob([data], {type: 'text/json'}),
        e    = document.createEvent('MouseEvents'),
        a    = document.createElement('a')

    a.download = filename
    a.href = window.URL.createObjectURL(blob)
    a.dataset.downloadurl =  ['text/json', a.download, a.href].join(':')
    e.initMouseEvent('click', true, false, window, 0, 0, 0, 0, 0, false, false, false, false, 0, null)
    a.dispatchEvent(e)
 }
})(console);

await (async function(console){
console.fetch = async function(exname){
res = await fetch("https://hdlbits.01xz.net/load.php", {
    "credentials": "include",
    "headers": {
        "User-Agent": "Mozilla/5.0 (X11; Linux x86_64; rv:78.0) Gecko/20100101 Firefox/78.0",
        "Accept": "*/*",
        "Accept-Language": "en-US,en;q=0.5",
        "Content-Type": "application/x-www-form-urlencoded"
    },
    "referrer": "https://hdlbits.01xz.net/wiki/",
    "body": "tc="+exname+"&name=0",
    "method": "POST",
    "mode": "cors"
});
res = await res.json();
console.save(res.data, exname);
}})(console);

$("a.vlgstat_link").toArray().forEach(e => console.fetch(e.text));
```
