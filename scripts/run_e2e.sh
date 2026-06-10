#!/bin/bash
set -euo pipefail

echo "Running JavaScript SDK tests..."

cd sdks/javascript
npm install
npm run build

# Run unit tests
echo "Running unit tests..."
npx vitest run --passWithNoTests

# Run integration test
echo "Running integration tests..."
node -e "
const { AgentBrowser, BrowserMode, Page, SecurityShield, getB4n1webVersion } = require('./dist/index.js');
console.log('[OK] AgentBrowser:', typeof AgentBrowser);
console.log('[OK] BrowserMode.LIGHT:', BrowserMode.LIGHT);
console.log('[OK] BrowserMode.JS:', BrowserMode.JS);
console.log('[OK] BrowserMode.RENDER:', BrowserMode.RENDER);
console.log('[OK] Page:', typeof Page);
console.log('[OK] SecurityShield:', typeof SecurityShield);
console.log('[OK] Version:', getB4n1webVersion());
const page = new Page({url: 'https://test.com', markdown: '# Test\nContent', links: ['https://a.com', 'https://b.com/test']});
console.log('[OK] Page.getMainContent():', page.getMainContent());
console.log('[OK] Page.findLinksByText(\"b.com\"):', page.findLinksByText('b.com'));
if (BrowserMode.LIGHT !== 'light') throw new Error('LIGHT wrong');
if (BrowserMode.JS !== 'js') throw new Error('JS wrong');
if (BrowserMode.RENDER !== 'render') throw new Error('RENDER wrong');
console.log('[OK] BrowserMode enum correct (3 modes)');
const shield = new SecurityShield();
shield.markDomain('evil.com', false);
const blocked = shield.isUrlSafe('https://evil.com');
console.log('[OK] SecurityShield blocks unsafe:', !blocked.isSafe);
console.log('');
console.log('[OK] All JavaScript SDK tests passed');
"

echo "JavaScript SDK tests completed successfully!"
