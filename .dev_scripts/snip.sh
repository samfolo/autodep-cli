#!/bin/bash

# TODO: delete this once things are rolling

output_file="output.txt"
directory="src/cli"

> "$output_file"

find "$directory" -type f | while read -r file; do
    echo "$file" >> "$output_file"
    echo '```' >> "$output_file"
    cat "$file" >> "$output_file"
    echo '```' >> "$output_file"
    echo "" >> "$output_file"
done

echo "Success: contents combined into $output_file"