import fs from 'fs';
import { initSync, score_json } from '../assets/wasm/dietarycodex.js';

const wasmB64 = fs.readFileSync(new URL('../assets/wasm/dietarycodex_bg.wasm.b64', import.meta.url), 'utf8');
initSync(Buffer.from(wasmB64, 'base64'));

const csv = fs.readFileSync(new URL('../data/template.csv', import.meta.url), 'utf8');
const lines = csv.trim().split(/\r?\n/);
const headers = lines[0].split(',');
const rows = lines.slice(1).map(line => {
  const values = line.split(',');
  const obj = {};
  headers.forEach((h, i) => {
    const v = values[i]?.trim();
    if (v === '') {
      obj[h] = null;
    } else {
      const num = Number(v);
      obj[h] = Number.isNaN(num) ? v : num;
    }
  });
  return obj;
});

const raw = score_json(JSON.stringify(rows));
const data = raw.rows.map(r => Object.fromEntries(r.scores));
console.log(JSON.stringify(data));
