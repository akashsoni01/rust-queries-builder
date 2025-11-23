#!/bin/bash
# Profiling script for rust-queries-builder
# On macOS, this requires sudo privileges for dtrace

set -e

EXAMPLE=${1:-comprehensive_i64_aggregators}
FEATURES=${2:-"datetime,parallel,parking_lot,tokio"}

echo "Profiling example: $EXAMPLE with features: $FEATURES"
echo "Building in release mode with debug symbols..."

# Build the example
cargo build --release --example "$EXAMPLE" --features "$FEATURES"

echo "Generating flamegraph..."
echo "Note: On macOS, this requires sudo privileges for dtrace"
echo "If you see dtrace errors, you may need to run: sudo ./profile.sh"

# Generate flamegraph
cargo flamegraph --example "$EXAMPLE" --features "$FEATURES" || {
    echo ""
    echo "Flamegraph generation failed. This is common on macOS due to System Integrity Protection."
    echo ""
    echo "Alternative options:"
    echo "1. Run with sudo: sudo ./profile.sh $EXAMPLE $FEATURES"
    echo "2. Use Instruments.app (Xcode):"
    echo "   - Open Instruments.app"
    echo "   - Select 'Time Profiler'"
    echo "   - Run: ./target/release/examples/$EXAMPLE"
    echo "3. Use sample command manually:"
    echo "   ./target/release/examples/$EXAMPLE &"
    echo "   sample \$! 10 -f /tmp/sample_output.txt"
    echo ""
    exit 1
}

echo ""
echo "Flamegraph generated: flamegraph.svg"
echo "Open it in your browser to view the profiling results."

