static mut v = vec![1, 2, 3];
static first = &v[0];
v.push(4);
println!("{first}");