#!/bin/bash

# Batch analysis script for cargo-geiger
# Analyzes all crate directories in the parent directory and generates a unified report

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GEIGER_BIN="$SCRIPT_DIR/target/debug/cargo-geiger"
PARENT_DIR="$(dirname "$SCRIPT_DIR")"
REPORT_FILE="$SCRIPT_DIR/unified_analysis_report.md"
TIMESTAMP=$(date "+%Y-%m-%d %H:%M:%S")

# Build cargo-geiger if not exists
if [ ! -f "$GEIGER_BIN" ]; then
    echo "Building cargo-geiger..."
    cd "$SCRIPT_DIR"
    cargo build --bin cargo-geiger
fi

# Initialize report
cat > "$REPORT_FILE" << EOF
# Unified Unsafe Code Analysis Report

**Generated:** $TIMESTAMP

This report contains unsafe code analysis results for all crates in \`$PARENT_DIR\`.

## Summary Table

| Crate | Functions | Expressions | Impls | Traits | Methods | Ptr Derefs | Unsafe Calls | Core | Alloc | Std | Other | Status |
|-------|-----------|-------------|-------|--------|---------|------------|--------------|------|-------|-----|-------|--------|
EOF

# Counter for statistics
total_crates=0
analyzed_crates=0
failed_crates=0
skipped_crates=0

# Temporary file for detailed reports
DETAILS_FILE=$(mktemp)

echo "Starting analysis of crates in $PARENT_DIR..."
echo ""

# Analyze each directory
for dir in "$PARENT_DIR"/*/; do
    dirname=$(basename "$dir")
    
    # Skip cargo-fingrain-geiger itself
    if [ "$dirname" = "cargo-fingrain-geiger" ]; then
        continue
    fi
    
    total_crates=$((total_crates + 1))
    
    # Check if it's a cargo project
    if [ ! -f "$dir/Cargo.toml" ]; then
        echo "⏭️  Skipping $dirname (no Cargo.toml)"
        skipped_crates=$((skipped_crates + 1))
        continue
    fi
    
    echo "📦 Analyzing $dirname..."
    
    # Run cargo-geiger and capture output
    cd "$dir"
    
    # Create a temp file for this crate's output
    OUTPUT_FILE=$(mktemp)
    
    if timeout 120 "$GEIGER_BIN" geiger --output-format Ascii 2>/dev/null > "$OUTPUT_FILE"; then
        analyzed_crates=$((analyzed_crates + 1))
        
        # Extract the summary line (last non-empty line with numbers)
        SUMMARY_LINE=$(grep -E "^[0-9]+/[0-9]+" "$OUTPUT_FILE" | tail -1 || echo "")
        
        if [ -n "$SUMMARY_LINE" ]; then
            # Parse the summary line
            # Format: Functions Expressions Impls Traits Methods PtrDerefs UnsafeCalls Core Alloc Std Other
            read -r funcs exprs impls traits methods ptr_derefs unsafe_calls core alloc std other <<< "$SUMMARY_LINE"
            
            # Determine status based on unsafe counts
            if echo "$SUMMARY_LINE" | grep -qE "^0/0\s+0/0\s+0/0\s+0/0\s+0/0\s+0/0\s+0/0"; then
                status="🔒 Safe"
            else
                status="☢️ Unsafe"
            fi
            
            # Add to summary table
            echo "| $dirname | $funcs | $exprs | $impls | $traits | $methods | $ptr_derefs | $unsafe_calls | $core | $alloc | $std | $other | $status |" >> "$REPORT_FILE"
        else
            echo "| $dirname | - | - | - | - | - | - | - | - | - | - | - | ⚠️ Parse Error |" >> "$REPORT_FILE"
        fi
        
        # Save detailed output
        echo "" >> "$DETAILS_FILE"
        echo "### $dirname" >> "$DETAILS_FILE"
        echo "" >> "$DETAILS_FILE"
        echo '```' >> "$DETAILS_FILE"
        cat "$OUTPUT_FILE" >> "$DETAILS_FILE"
        echo '```' >> "$DETAILS_FILE"
        
        echo "   ✅ Done"
    else
        failed_crates=$((failed_crates + 1))
        echo "| $dirname | - | - | - | - | - | - | - | - | - | - | - | ❌ Failed |" >> "$REPORT_FILE"
        echo "   ❌ Failed (timeout or error)"
    fi
    
    rm -f "$OUTPUT_FILE"
done

# Add statistics section
cat >> "$REPORT_FILE" << EOF

## Statistics

- **Total Directories:** $total_crates
- **Successfully Analyzed:** $analyzed_crates
- **Failed:** $failed_crates
- **Skipped (no Cargo.toml):** $skipped_crates

## Detailed Reports

EOF

# Append detailed reports
cat "$DETAILS_FILE" >> "$REPORT_FILE"
rm -f "$DETAILS_FILE"

echo ""
echo "============================================"
echo "Analysis Complete!"
echo "============================================"
echo "Total: $total_crates | Analyzed: $analyzed_crates | Failed: $failed_crates | Skipped: $skipped_crates"
echo ""
echo "Report saved to: $REPORT_FILE"
