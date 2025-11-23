# Profiling Guide

This guide explains how to profile the rust-queries-builder codebase.

## Prerequisites

1. **Install flamegraph:**
   ```bash
   cargo install flamegraph
   ```

2. **Enable debug symbols** (already configured in `Cargo.toml`):
   ```toml
   [profile.release]
   debug = true
   ```

3. **Enable frame pointers** (recommended for better stack traces):
   ```bash
   export RUSTFLAGS="-C force-frame-pointers=yes"
   ```

## macOS Profiling

On macOS, `cargo flamegraph` uses `dtrace` which requires special privileges due to System Integrity Protection (SIP).

### Option 1: Run with sudo (Recommended)

```bash
sudo ./profile.sh comprehensive_i64_aggregators "datetime,parallel,parking_lot,tokio"
```

Or manually:
```bash
sudo cargo flamegraph --example comprehensive_i64_aggregators --features datetime,parallel,parking_lot,tokio
```

### Option 2: Use Instruments.app (Xcode)

1. Open **Instruments.app** (comes with Xcode)
2. Select **Time Profiler**
3. Choose your target: `./target/release/examples/comprehensive_i64_aggregators`
4. Click **Record** to start profiling
5. View the call tree and time profiler results

### Option 3: Use sample command

```bash
# Build the example
cargo build --release --example comprehensive_i64_aggregators --features datetime,parallel,parking_lot,tokio

# Run and sample
./target/release/examples/comprehensive_i64_aggregators &
PID=$!
sample $PID 10 -f /tmp/sample_output.txt
wait $PID

# View the output
cat /tmp/sample_output.txt
```

### Option 4: Use cargo-profdata (if available)

```bash
cargo install cargo-profdata
cargo profdata --example comprehensive_i64_aggregators --features datetime,parallel,parking_lot,tokio
```

## Linux Profiling

On Linux, `cargo flamegraph` uses `perf`:

```bash
# Install perf (if not already installed)
sudo apt install linux-perf  # Debian/Ubuntu
# or
sudo yum install perf         # RHEL/CentOS

# Run profiling
cargo flamegraph --example comprehensive_i64_aggregators --features datetime,parallel,parking_lot,tokio
```

## Available Examples for Profiling

- `comprehensive_i64_aggregators` - Comprehensive aggregator operations
- `lazy_parallel_performance_comparison` - Performance comparison
- `lazy_parallel_query_demo` - Parallel lazy queries
- `parallel_queries_demo` - Parallel query operations

## Viewing Results

After profiling, a `flamegraph.svg` file will be generated in the project root. Open it in your web browser to view:

- **Width** = Time spent in function
- **Height** = Call stack depth
- **Click** on functions to zoom in
- **Search** for specific function names

## Tips

1. **Profile release builds** - Always profile optimized release builds for realistic performance data
2. **Run long enough** - Ensure your program runs long enough to collect meaningful data (at least a few seconds)
3. **Focus on hot paths** - Look for wide bars in the flamegraph - these are your performance bottlenecks
4. **Compare before/after** - Generate flamegraphs before and after optimizations to measure improvements

