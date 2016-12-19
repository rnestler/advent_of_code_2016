extern crate puzzle12;

fn main() {
    let input_part1 =
"cpy 1 a
cpy 1 b
cpy 26 d
jnz c 2
jnz 1 5
cpy 7 c
inc d
dec c
jnz c -2
cpy a c
inc a
dec b
jnz b -2
cpy c b
dec d
jnz d -6
cpy 14 c
cpy 14 d
inc a
dec d
jnz d -2
dec c
jnz c -5";
    let result_part1 = puzzle12::puzzle(input_part1);
    println!("{}", result_part1);
    
    let input_part2 = "cpy 1 c\n".to_string() + input_part1;
    let result_part2 = puzzle12::puzzle(&input_part2);
    println!("{}", result_part2);
}
