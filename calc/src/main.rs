use std::io;
fn main()
{
	let mut choice=String::new();
	println!("Menu:\n1.Addition.\n2.Subtraction.\n3.Multiplication.\n4.Division.\nEnter your choice:");
	io::stdin().read_line(&mut choice)
	.ok()
	.expect("Failed to read line");
	let mut a=String::new();
	let mut b=String::new();
	println!("Enter two numbers:");
	io::stdin().read_line(&mut a)
	.ok()
	.expect("Failed to read line");
	io::stdin().read_line(&mut b)
	.ok()
	.expect("Failed to read line");
	let a:u32=a.trim().parse()
	.ok()
	.expect("Please enter a number");
	let b:u32=b.trim().parse()
	.ok()
	.expect("Please enter a number");
	let choice:u32=choice.trim().parse()
	.ok()
	.expect("Please enter a valid choice");
	if choice==1
	{
		println!("SUM: {}",a+b);
	}
	else if choice==2
	{
		println!("DIFFERENCE: {}",a-b);
	}
	else if choice==3
	{
		println!("PRODUCT: {}",a*b);
	}
	else if choice==4
	{
		println!("QUOTIENT: {}",a/b);
	}
}
	