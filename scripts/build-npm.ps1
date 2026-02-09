#!/usr/bin/env pwsh

<#
.SYNOPSIS
    本地构建并准备 npm 包

.DESCRIPTION
    为当前平台构建 Rust 二进制，并复制到对应的 npm 目录

.EXAMPLE
    .\scripts\build-npm.ps1
#>

$ErrorActionPreference = "Stop"

# 获取当前平台信息
$platform = if ($IsWindows -or $env:OS -eq "Windows_NT") { "win32" }
            elseif ($IsMacOS) { "darwin" }
            else { "linux" }

$arch = if ([System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture -eq "Arm64") { "arm64" }
        else { "x64" }

$npmDir = "$platform-$arch"
$binaryName = if ($platform -eq "win32") { "skills-scanner.exe" } else { "skills-scanner" }

Write-Host "Building for $platform-$arch..." -ForegroundColor Cyan

# 构建 Rust 项目
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Error "Cargo build failed"
    exit 1
}

# 创建目标目录
$targetDir = Join-Path $PSScriptRoot ".." "npm" $npmDir "bin"
New-Item -ItemType Directory -Force -Path $targetDir | Out-Null

# 复制二进制文件
$sourcePath = Join-Path $PSScriptRoot ".." "target" "release" $binaryName
$destPath = Join-Path $targetDir $binaryName

Copy-Item -Path $sourcePath -Destination $destPath -Force

Write-Host "✓ Binary copied to npm/$npmDir/bin/$binaryName" -ForegroundColor Green
Write-Host ""
Write-Host "To test locally:" -ForegroundColor Yellow
Write-Host "  node bin/cli.js --help"
Write-Host ""
Write-Host "To publish:" -ForegroundColor Yellow
Write-Host "  cd npm/$npmDir && npm publish --access public"
Write-Host "  npm publish --access public"
