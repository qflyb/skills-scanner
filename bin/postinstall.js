#!/usr/bin/env node

/**
 * postinstall 脚本
 * 验证平台二进制是否正确安装
 */

const fs = require('fs');
const path = require('path');

const platform = process.platform;
const arch = process.arch;

const platformMap = {
  'win32': 'win32',
  'darwin': 'darwin',
  'linux': 'linux'
};

const archMap = {
  'x64': 'x64',
  'arm64': 'arm64'
};

const platformKey = platformMap[platform];
const archKey = archMap[arch];

if (!platformKey || !archKey) {
  console.warn(`\n⚠️  skills-scanner: Unsupported platform ${platform}-${arch}`);
  console.warn('   You may need to build from source.\n');
  process.exit(0);
}

const packageName = `@skills-scanner/${platformKey}-${archKey}`;

try {
  require.resolve(packageName);
  console.log(`✓ skills-scanner: Binary installed for ${platform}-${arch}`);
} catch (e) {
  console.warn(`\n⚠️  skills-scanner: Optional dependency ${packageName} not installed.`);
  console.warn('   This is expected if npm install was run with --ignore-optional.\n');
}
