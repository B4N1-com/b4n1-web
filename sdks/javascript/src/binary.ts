import * as os from 'os';
import * as fs from 'fs';
import * as path from 'path';
import { execSync } from 'child_process';

const PLATFORM_BINARIES: Record<string, string> = {
  'linux-x64': 'b4n1web-linux-amd64',
  'linux-arm64': 'b4n1web-linux-arm64',
  'darwin-x64': 'b4n1web-macos-x64',
  'darwin-arm64': 'b4n1web-macos-arm64',
  'win32-x64': 'b4n1web-windows-amd64.exe',
  'win32-arm64': 'b4n1web-windows-arm64.exe',
};

function getPlatformBinaryName(): string | null {
  const plat = os.platform();
  const arch = os.arch();
  return PLATFORM_BINARIES[`${plat}-${arch}`] ?? null;
}

export function getB4n1webBinary(): string | null {
  const envPath = process.env.B4N1WEB_BIN_PATH;
  if (envPath) {
    try {
      fs.accessSync(envPath, fs.constants.X_OK);
      return envPath;
    } catch {}
  }

  const binName = getPlatformBinaryName();
  if (binName) {
    const bundled = path.join(__dirname, '..', 'bin', binName);
    try {
      fs.chmodSync(bundled, 0o755);
    } catch {}
    try {
      fs.accessSync(bundled, fs.constants.X_OK);
      return bundled;
    } catch {}
  }

  const home = os.homedir();
  const paths: string[] = [];

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
    } catch {}
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

const SDK_VERSION = '0.13.0';
