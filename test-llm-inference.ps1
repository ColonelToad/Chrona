#!/usr/bin/env pwsh
# Quick test of LLM inference subprocess execution

$ModelPath = "C:\Users\legot\Chrona\data\mini\model-3b.llamafile"
$TestPrompt = "What should I do about elevated heart rate during rest?"

if (-not (Test-Path $ModelPath)) {
    Write-Error "Model file not found at: $ModelPath"
    exit 1
}

Write-Host "Testing LLM inference..." -ForegroundColor Cyan
Write-Host "Model: $ModelPath" -ForegroundColor Gray
Write-Host "Prompt: '$TestPrompt'" -ForegroundColor Gray
Write-Host "---" -ForegroundColor Gray

# Run the .llamafile with inference parameters
$output = & $ModelPath -c 256 -n 64 -p $TestPrompt -t 4 2>&1

Write-Host "Response:" -ForegroundColor Cyan
Write-Host $output -ForegroundColor Green

Write-Host "---" -ForegroundColor Gray
Write-Host "Inference test complete!" -ForegroundColor Cyan
