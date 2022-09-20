var fs = require('fs');
var path = require('path');
const VERBOSE_LOGS = process.env.NODE_LOG == '1';

function randomInt(min, max) {  
  return Math.floor(
    Math.random() * (max - min) + min
  )
}

async function log(){
    const dateTimeStamp = new Date().toISOString();
    const tokenValue = randomInt(9999999999, 1111111111);
    if (VERBOSE_LOGS) {
      fs.appendFile(path.join('/var/log/badnode.log'), `${dateTimeStamp} token: ${tokenValue}` + '\n', (err) => {
          if (err) throw err;
      }); 
    }
}
setInterval(log, 1000);
