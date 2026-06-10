const { AgentBrowser, BrowserMode, BinaryNotFoundError, NavigationError } = require('../dist/index.js');

async function main() {
  // Check binary location
  const binaryPath = AgentBrowser.findBinary();
  if (!binaryPath) {
    console.error('b4n1web binary not found. Install: curl -sL https://web.b4n1.com/install | bash');
    process.exit(1);
  }
  console.log('Binary found at:', binaryPath);

  // Example 1: Light mode (fastest, no JS)
  console.log('\n--- Light Mode ---');
  try {
    const browser = new AgentBrowser({ mode: BrowserMode.LIGHT });
    const page = await browser.goto('https://example.com');
    console.log('URL:', page.url);
    console.log('Title:', page.getMainContent().split('\n')[0]);
    console.log('Links:', page.links.length);
    console.log('First link:', page.findLinksByText('more')[0] || 'none');
    browser.close();
  } catch (err) {
    console.error('Light mode error:', err.message);
  }

  // Example 2: JS mode with waitFor
  console.log('\n--- JS Mode with waitFor ---');
  try {
    const browser = new AgentBrowser({ mode: BrowserMode.JS, timeout: 15 });
    const page = await browser.goto('https://example.com', 'body');
    console.log('JS Output:', page.jsOutput ? 'present' : 'none');
    browser.close();
  } catch (err) {
    console.error('JS mode error:', err.message);
  }

  // Example 3: Render mode and interactions
  console.log('\n--- Render Mode ---');
  try {
    const browser = new AgentBrowser({ mode: BrowserMode.RENDER, timeout: 30 });
    const page = await browser.goto('https://example.com');
    console.log('Screenshot:', page.screenshot ? page.screenshot.substring(0, 40) + '...' : 'none');

    // Wait for an element
    const found = await browser.waitForSelector('h1', 5000);
    console.log('h1 found:', found);

    // Get all links
    const links = browser.getLinks();
    console.log('Total links:', links.length);

    // Screenshot with custom dimensions
    try {
      const ss = browser.screenshot(800, 600);
      console.log('Custom screenshot:', ss.substring(0, 40) + '...');
    } catch (ssErr) {
      console.log('Screenshot requires render mode binary support');
    }

    browser.close();
  } catch (err) {
    if (err instanceof BinaryNotFoundError) {
      console.error('Binary not installed. Skipping render mode tests.');
    } else if (err instanceof NavigationError) {
      console.error('Navigation failed:', err.message);
    } else {
      console.error('Render mode error:', err.message);
    }
  }

  // Example 4: Static utility - get links from page
  console.log('\n--- getLinksFromPage ---');
  try {
    const links = await AgentBrowser.getLinksFromPage('https://example.com', BrowserMode.LIGHT);
    console.log('Links from static method:', links.length);
    links.forEach((link, i) => console.log(`  [${i}] ${link}`));
  } catch (err) {
    console.error('getLinksFromPage error:', err.message);
  }

  console.log('\nDone.');
}

main().catch(console.error);
