#!/bin/bash

# --- Configuration ---

# Default to current year and day if not provided as arguments
# Example usage: ./download_aoc_input.sh 2023 1
YEAR=${1:-$(date +%Y)}
DAY=${2:-$(date +%d)}

# Check for the required session cookie
if [ -z "${AOC_SESSION}" ]; then
  echo "Error: AOC_SESSION environment variable is not set." >&2
  echo "Please set it with the value of your 'session' cookie from adventofcode.com." >&2
  exit 1
fi

# Ensure day is between 1 and 25
if (( DAY < 1 || DAY > 25 )); then
    echo "Error: Day must be between 1 and 25." >&2
    exit 1
fi

# Pad day with a leading zero if it's a single digit (for file naming)
PADDED_DAY=$(printf "%02d" $DAY)

# Define the output file name
OUTPUT_FILE="${YEAR}/${PADDED_DAY}/input.txt"

# --- Download Logic ---

# Create the year directory if it doesn't exist
mkdir -p "${YEAR}/${PADDED_DAY}"

echo "Attempting to download input for Year ${YEAR}, Day ${DAY} to ${OUTPUT_FILE}..."

# Construct the URL
URL="https://adventofcode.com/${YEAR}/day/${DAY}/input"

# Use curl to download the input
# -s: Silent mode
# -b: Pass the cookie string
# -o: Output to the specified file
curl -s -b "session=${AOC_SESSION}" "${URL}" -o "${OUTPUT_FILE}"

# Check the curl exit status
if [ $? -eq 0 ]; then
    # Check if the downloaded file is empty (AoC returns a non-200 code for locked puzzles/invalid cookies, but sometimes an empty file on redirects)
    if [ -s "${OUTPUT_FILE}" ]; then
        echo "✅ Download successful!"
    else
        echo "❌ Download failed or file is empty. Check your AOC_SESSION cookie and ensure the puzzle for Day ${DAY} is unlocked." >&2
        rm "${OUTPUT_FILE}" # Clean up empty file
        exit 1
    fi
else
    echo "❌ curl command failed." >&2
    exit 1
fi
