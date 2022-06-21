struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

struct NumDrop;

impl Drop for NumDrop {
    fn drop(&mut self) {
        println!("NumDrop is being dropped");
    }
}

fn main() {
    let x = ToDrop;
    
    for n in 1..=10 {
        println!("{}",n);
        let y = NumDrop;
    }
}
