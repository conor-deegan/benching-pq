#!/bin/bash

echo "=== System Information ==="
echo "OS: $(sw_vers -productName) $(sw_vers -productVersion)"
echo "Kernel: $(uname -r)"
echo "Architecture: $(uname -m)"

echo -e "\n=== CPU Information ==="
sysctl -n machdep.cpu.brand_string
echo "Cores: $(sysctl -n machdep.cpu.core_count)"
echo "Threads: $(sysctl -n machdep.cpu.thread_count)"

echo -e "\n=== Memory Information ==="
sysctl -n hw.memsize | awk '{print $0/1024/1024/1024 " GB"}'

echo -e "\n=== Rust Information ==="
rustc --version
cargo --version

echo -e "\n=== Benchmark Environment ==="
echo "Date: $(date)"
echo "Hostname: $(hostname)" 