import * as os from 'os';
import * as fs from 'fs';
import * as path from 'path';
import { execSync } from 'child_process';

export function getB4n1webBinary(): string | null {
  const home = os.homedir();
  const paths: string[] = [];

  const bundledBinary = path.join(__dirname, '..', 'bin', 'b4n1web-linux');
  if (fs.existsSync(bundledBinary)) {
    try {
      fs.accessSync(bundledBinary, fs.constants.X_OK);
      return bundledBinary;
    } catch {
    }
  }

  const envBinary = process.env.B4N1WEB_BINARY;
  if (envBinary) paths.push(envBinary);

  paths.push(
    home + '/.local/bin/b4n1web',
    home + '/.b4n1web/bin/b4n1web',
    '/usr/local/bin/b4n1web',
    '/usr/bin/b4n1web',
  );

  const pathEnv = process.env.PATH || '';
  const pathDirs = pathEnv.split(':').filter(p => p);
  for (const dir of pathDirs) {
    paths.push(dir + '/b4n1web');
  }

  for (const filePath of paths) {
    try {
      if (fs.existsSync(filePath)) {
        const stats = fs.statSync(filePath);
        if (stats.isFile() && (stats.mode & 0o111) !== 0) {
          return filePath;
        }
      }
    } catch {
    }
  }
  return null;
}

export function getB4n1webVersion(): string | null {
  const binaryPath = getB4n1webBinary();
  if (!binaryPath) {
    return null;
  }
  try {
    const version = execSync(`${binaryPath} --version`, { timeout: 5000 }).toString().trim();
    const parts = version.split(' ');
    if (parts.length >= 2 && parts[0] === 'b4n1web') {
      return parts[1];
    }
    return null;
  } catch {
    return null;
  }
}

const SDK_VERSION = '0.8.0';

export function checkVersionCompatibility(): string | null {
  const binaryVersion = getB4n1webVersion();
  if (!binaryVersion) {
    return null;
  }

  if (binaryVersion !== SDK_VERSION) {
    console.warn(
      `Warning: Version mismatch: SDK v${SDK_VERSION} requires binary v${SDK_VERSION}, ` +
      `but found v${binaryVersion}. Some features may not work correctly. ` +
      `To update: curl -sL https://web.b4n1.com/install | bash`
    );
  }
  return binaryVersion;
}
