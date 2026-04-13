#!/usr/bin/env node
/**
 * Post-install script: Make bundled binary executable
 */
const fs = require('fs');
const path = require('path');

const binDir = path.join(__dirname, '..', 'bin');
const binary = path.join(binDir, 'b4n1web-linux');

if (fs.existsSync(binary)) {
  try {
    fs.chmodSync(binary, 0o755);
    console.log('✅ b4n1web binary installed');
  } catch (err) {
    console.warn('⚠️  Could not chmod binary:', err.message);
  }
}
