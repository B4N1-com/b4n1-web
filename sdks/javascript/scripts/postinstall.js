#!/usr/bin/env node
/**
 * Post-install script: Make all bundled binaries executable
 */
const fs = require('fs');
const path = require('path');

const binDir = path.join(__dirname, '..', 'bin');
const binaries = [
  'b4n1web-linux-amd64',
  'b4n1web-linux-arm64',
  'b4n1web-macos-x64',
  'b4n1web-macos-arm64',
];

for (const name of binaries) {
  const p = path.join(binDir, name);
  if (fs.existsSync(p)) {
    try {
      fs.chmodSync(p, 0o755);
    } catch {}
  }
}
