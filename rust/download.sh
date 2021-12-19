#!/bin/bash

set -e;

if ! command -v 'aoc' &> /dev/null
then
    echo "command \`aoc\` not found. Try running \`cargo install aoc-cli\` to install it."
    exit 1
fi

if [ $# -lt 2 ]; then
    >&2 echo "Arguments are required for a year and a day."
    exit 1
fi

year=$(echo $1);
day=$(echo $2 | sed 's/^0*//');
day_padded=`printf %02d $day`;

filename="day$day_padded";

input_path="$year/src/input/$filename.txt";

tmp_dir=$(mktemp -d);
tmp_file_path="$tmp_dir/input";

aoc download --day $day --file $tmp_file_path;
cat $tmp_file_path > $input_path;
echo "Wrote input to \`$input_path\`...";

echo "Have fun! 🎄";

# Make sure it gets removed even if the script exits abnormally.
trap "exit 1"           HUP INT PIPE QUIT TERM
trap 'rm -rf "$tmp_dir"' EXIT
