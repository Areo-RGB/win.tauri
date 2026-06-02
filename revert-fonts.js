const fs = require('fs');
const path = require('path');

const cssPath = path.join(__dirname, 'src', 'index.css');
const css = fs.readFileSync(cssPath, 'utf8');
fs.writeFileSync(cssPath, css.replace(/font-size:\s*11px/g, 'font-size: 10px'));
console.log('Font bump reverted.');
