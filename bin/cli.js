#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

const BINARY_NAME = 'skills-scanner';

function getBinaryPath() {
  const platform = process.platform;
  const arch = process.arch;
  
  // 平台映射
  const platformMap = {
    'win32': 'win32',
    'darwin': 'darwin',
    'linux': 'linux'
  };
  
  // 架构映射
  const archMap = {
    'x64': 'x64',
    'arm64': 'arm64'
  };
  
  const platformKey = platformMap[platform];
  const archKey = archMap[arch];
  
  if (!platformKey || !archKey) {
    throw new Error(`Unsupported platform: ${platform}-${arch}`);
  }
  
  const packageName = `@skills-scanner/${platformKey}-${archKey}`;
  
  try {
    // 尝试从可选依赖中加载
    const binaryPackage = require.resolve(`${packageName}/package.json`);
    const binaryDir = path.dirname(binaryPackage);
    const binaryName = platform === 'win32' ? `${BINARY_NAME}.exe` : BINARY_NAME;
    const binaryPath = path.join(binaryDir, 'bin', binaryName);
    
    if (fs.existsSync(binaryPath)) {
      return binaryPath;
    }
  } catch (e) {
    // 可选依赖未安装
  }
  
  // 回退：检查本地开发二进制
  const localBinary = path.join(__dirname, '..', 'target', 'release', 
    platform === 'win32' ? `${BINARY_NAME}.exe` : BINARY_NAME);
  
  if (fs.existsSync(localBinary)) {
    return localBinary;
  }
  
  throw new Error(
    `Could not find binary for ${platform}-${arch}. ` +
    `Please ensure @skills-scanner/${platformKey}-${archKey} is installed.`
  );
}

function run() {
  const binaryPath = getBinaryPath();
  
  const child = spawn(binaryPath, process.argv.slice(2), {
    stdio: 'inherit',
    shell: false
  });
  
  child.on('error', (err) => {
    console.error(`Failed to start skills-scanner: ${err.message}`);
    process.exit(1);
  });
  
  child.on('close', (code) => {
    process.exit(code ?? 0);
  });
}

run();
