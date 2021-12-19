#!/bin/bash

set -e;

if [ $# -lt 2 ]; then
    >&2 echo "Arguments are required for a year and a day."
    exit 1
fi

year=$(echo $1);
day=$(echo $2 | sed 's/^0*//');
day_padded=`printf %02d $day`;

filename="day$day_padded";

input_path="$year/src/input/$filename.txt";
example_path="$year/src/examples/$filename.txt";
module_path="$year/src/solutions/$filename.rs";

touch $module_path;
echo "Created module \`$module_path\`";

cat > "$year/src/solutions/$filename.rs" <<EOF
pub fn part_one(input: &str) -> u32 {
    0
}

pub fn part_two(input: &str) -> u32 {
    0
}

#[test]
fn test_part_one() {
    use aoc::utils::read_file;
    let input = read_file("examples", day);
    assert_eq!(part_one(&input), 0);
}

#[test]
fn test_part_two() {
    use aoc::utils::read_file;
    let input = read_file("examples", day);
    assert_eq!(part_two(&input), 0);
}
EOF

perl -pi -e "s,day,$day,g" "$year/src/solutions/$filename.rs";

touch $input_path;
echo "Created input file \`$input_path\`";

touch $example_path;
echo "Created example file \`$example_path\`";

line="        $day => aoc::solve_day!($filename, &input),"
perl -pi -le "print '$line' if(/^*.day not solved/);" "$year/src/main.rs";

echo "Linked new module in \`src/main.rs\`";

LINE="pub mod $filename;";
FILE="$year/src/solutions/mod.rs";
grep -qF -- "$LINE" "$FILE" || echo "$LINE" >> "$FILE";
echo "Linked new module in \`$FILE\`";

echo "Have fun! 🎄";
