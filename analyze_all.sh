#!/bin/bash
# Analyze all cargo projects in /home/crates/source/ for unsafe usage

set -e

SOURCE_DIR="/home/crates/source"
CARGO_GEIGER="$SOURCE_DIR/cargo-fingrain-geiger/target/debug/cargo-geiger"
RESULTS_FILE="unsafe_analysis_results.csv"
SUMMARY_FILE="unsafe_analysis_summary.txt"

echo "═══════════════════════════════════════════════════════════"
echo "  Unsafe Usage Analysis - All Cargo Projects"
echo "═══════════════════════════════════════════════════════════"
echo ""

# Check if cargo-geiger is built
if [ ! -f "$CARGO_GEIGER" ]; then
    echo "Building cargo-geiger..."
    cd "$SOURCE_DIR/cargo-fingrain-geiger"
    cargo build --quiet
    cd -
fi

echo "Discovering cargo projects..."
PROJECTS=$(find "$SOURCE_DIR" -maxdepth 2 -name "Cargo.toml" -type f)
PROJECT_COUNT=$(echo "$PROJECTS" | wc -l)
echo "Found $PROJECT_COUNT cargo projects"
echo ""

# Initialize CSV
echo "Project,Functions_Unsafe,Functions_Total,Expressions_Unsafe,Expressions_Total,Impls_Unsafe,Impls_Total,Traits_Unsafe,Traits_Total,Methods_Unsafe,Methods_Total,PtrDerefs_Unsafe,PtrDerefs_Total,UnsafeCalls_Unsafe,UnsafeCalls_Total,Forbids_Unsafe" > "$RESULTS_FILE"

# Counters
SUCCESSFUL=0
FAILED=0

# Aggregates
TOTAL_FUNC_UNSAFE=0
TOTAL_FUNC_TOTAL=0
TOTAL_EXPR_UNSAFE=0
TOTAL_EXPR_TOTAL=0
TOTAL_IMPL_UNSAFE=0
TOTAL_IMPL_TOTAL=0
TOTAL_TRAIT_UNSAFE=0
TOTAL_TRAIT_TOTAL=0
TOTAL_METHOD_UNSAFE=0
TOTAL_METHOD_TOTAL=0
TOTAL_PTR_UNSAFE=0
TOTAL_PTR_TOTAL=0
TOTAL_CALL_UNSAFE=0
TOTAL_CALL_TOTAL=0

echo "Starting analysis..."
echo ""

# Analyze each project
while IFS= read -r cargo_toml; do
    PROJECT_DIR=$(dirname "$cargo_toml")
    PROJECT_NAME=$(basename "$PROJECT_DIR")

    echo -n "Analyzing: $PROJECT_NAME ... "

    # Run cargo-geiger
    cd "$PROJECT_DIR"
    OUTPUT=$("$CARGO_GEIGER" geiger --quiet 2>&1 || true)

    # Extract the stats line (contains slashes and numbers)
    STATS_LINE=$(echo "$OUTPUT" | grep -E '^[0-9]+/[0-9]+' | head -1 || true)

    if [ -n "$STATS_LINE" ]; then
        # Parse the line: "1/1  2/2  0/0  0/0  0/0  2/2  0/0  ! crate_name"
        IFS=' ' read -ra PARTS <<< "$STATS_LINE"

        if [ ${#PARTS[@]} -ge 7 ]; then
            # Extract fractions
            IFS='/' read -r FUNC_U FUNC_T <<< "${PARTS[0]}"
            IFS='/' read -r EXPR_U EXPR_T <<< "${PARTS[1]}"
            IFS='/' read -r IMPL_U IMPL_T <<< "${PARTS[2]}"
            IFS='/' read -r TRAIT_U TRAIT_T <<< "${PARTS[3]}"
            IFS='/' read -r METHOD_U METHOD_T <<< "${PARTS[4]}"
            IFS='/' read -r PTR_U PTR_T <<< "${PARTS[5]}"
            IFS='/' read -r CALL_U CALL_T <<< "${PARTS[6]}"

            # Check forbids unsafe
            FORBIDS="No"
            if [ "${PARTS[7]}" = ":)" ]; then
                FORBIDS="Yes"
            fi

            # Write to CSV
            echo "$PROJECT_NAME,$FUNC_U,$FUNC_T,$EXPR_U,$EXPR_T,$IMPL_U,$IMPL_T,$TRAIT_U,$TRAIT_T,$METHOD_U,$METHOD_T,$PTR_U,$PTR_T,$CALL_U,$CALL_T,$FORBIDS" >> "$RESULTS_FILE"

            # Aggregate
            TOTAL_FUNC_UNSAFE=$((TOTAL_FUNC_UNSAFE + FUNC_U))
            TOTAL_FUNC_TOTAL=$((TOTAL_FUNC_TOTAL + FUNC_T))
            TOTAL_EXPR_UNSAFE=$((TOTAL_EXPR_UNSAFE + EXPR_U))
            TOTAL_EXPR_TOTAL=$((TOTAL_EXPR_TOTAL + EXPR_T))
            TOTAL_IMPL_UNSAFE=$((TOTAL_IMPL_UNSAFE + IMPL_U))
            TOTAL_IMPL_TOTAL=$((TOTAL_IMPL_TOTAL + IMPL_T))
            TOTAL_TRAIT_UNSAFE=$((TOTAL_TRAIT_UNSAFE + TRAIT_U))
            TOTAL_TRAIT_TOTAL=$((TOTAL_TRAIT_TOTAL + TRAIT_T))
            TOTAL_METHOD_UNSAFE=$((TOTAL_METHOD_UNSAFE + METHOD_U))
            TOTAL_METHOD_TOTAL=$((TOTAL_METHOD_TOTAL + METHOD_T))
            TOTAL_PTR_UNSAFE=$((TOTAL_PTR_UNSAFE + PTR_U))
            TOTAL_PTR_TOTAL=$((TOTAL_PTR_TOTAL + PTR_T))
            TOTAL_CALL_UNSAFE=$((TOTAL_CALL_UNSAFE + CALL_U))
            TOTAL_CALL_TOTAL=$((TOTAL_CALL_TOTAL + CALL_T))

            echo "✓"
            SUCCESSFUL=$((SUCCESSFUL + 1))
        else
            echo "✗ (parse error)"
            FAILED=$((FAILED + 1))
        fi
    else
        echo "✗ (no output)"
        FAILED=$((FAILED + 1))
    fi
done <<< "$PROJECTS"

# Generate summary
{
    echo "═══════════════════════════════════════════════════════════"
    echo "  AGGREGATE STATISTICS"
    echo "═══════════════════════════════════════════════════════════"
    echo ""
    echo "Successfully analyzed: $SUCCESSFUL"
    echo "Failed to analyze: $FAILED"
    echo ""
    echo "┌─────────────────────┬──────────┬──────────┬────────────┐"
    echo "│ Metric              │ Unsafe   │ Total    │ Percentage │"
    echo "├─────────────────────┼──────────┼──────────┼────────────┤"

    calc_percent() {
        if [ "$2" -gt 0 ]; then
            echo "scale=2; ($1 * 100) / $2" | bc
        else
            echo "0.00"
        fi
    }

    printf "│ %-19s │ %8d │ %8d │ %9.2f%% │\n" "Functions" $TOTAL_FUNC_UNSAFE $TOTAL_FUNC_TOTAL $(calc_percent $TOTAL_FUNC_UNSAFE $TOTAL_FUNC_TOTAL)
    printf "│ %-19s │ %8d │ %8d │ %9.2f%% │\n" "Expressions" $TOTAL_EXPR_UNSAFE $TOTAL_EXPR_TOTAL $(calc_percent $TOTAL_EXPR_UNSAFE $TOTAL_EXPR_TOTAL)
    printf "│ %-19s │ %8d │ %8d │ %9.2f%% │\n" "Impls" $TOTAL_IMPL_UNSAFE $TOTAL_IMPL_TOTAL $(calc_percent $TOTAL_IMPL_UNSAFE $TOTAL_IMPL_TOTAL)
    printf "│ %-19s │ %8d │ %8d │ %9.2f%% │\n" "Traits" $TOTAL_TRAIT_UNSAFE $TOTAL_TRAIT_TOTAL $(calc_percent $TOTAL_TRAIT_UNSAFE $TOTAL_TRAIT_TOTAL)
    printf "│ %-19s │ %8d │ %8d │ %9.2f%% │\n" "Methods" $TOTAL_METHOD_UNSAFE $TOTAL_METHOD_TOTAL $(calc_percent $TOTAL_METHOD_UNSAFE $TOTAL_METHOD_TOTAL)
    printf "│ %-19s │ %8d │ %8d │ %9.2f%% │\n" "Ptr Derefs" $TOTAL_PTR_UNSAFE $TOTAL_PTR_TOTAL $(calc_percent $TOTAL_PTR_UNSAFE $TOTAL_PTR_TOTAL)
    printf "│ %-19s │ %8d │ %8d │ %9.2f%% │\n" "Unsafe Calls" $TOTAL_CALL_UNSAFE $TOTAL_CALL_TOTAL $(calc_percent $TOTAL_CALL_UNSAFE $TOTAL_CALL_TOTAL)

    echo "└─────────────────────┴──────────┴──────────┴────────────┘"
    echo ""

    TOTAL_UNSAFE=$((TOTAL_FUNC_UNSAFE + TOTAL_EXPR_UNSAFE + TOTAL_IMPL_UNSAFE + TOTAL_TRAIT_UNSAFE + TOTAL_METHOD_UNSAFE + TOTAL_PTR_UNSAFE + TOTAL_CALL_UNSAFE))
    TOTAL_ITEMS=$((TOTAL_FUNC_TOTAL + TOTAL_EXPR_TOTAL + TOTAL_IMPL_TOTAL + TOTAL_TRAIT_TOTAL + TOTAL_METHOD_TOTAL + TOTAL_PTR_TOTAL + TOTAL_CALL_TOTAL))

    echo "TOTAL UNSAFE USAGE: $TOTAL_UNSAFE out of $TOTAL_ITEMS items ($(calc_percent $TOTAL_UNSAFE $TOTAL_ITEMS)%)"
    echo ""
    echo "═══════════════════════════════════════════════════════════"
    echo "  Results saved to:"
    echo "    - $RESULTS_FILE (detailed CSV)"
    echo "    - $SUMMARY_FILE (this summary)"
    echo "═══════════════════════════════════════════════════════════"
} | tee "$SUMMARY_FILE"

echo ""
echo "Analysis complete!"
