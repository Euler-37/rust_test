use std::io;
const MAXSTACK:usize=1000;
fn main() {
    let mut data:[u32;MAXSTACK]=[0;MAXSTACK];
    let mut code=String::new();
    io::stdin().read_line(&mut code).expect("Failed to read code");
    let lens=code.chars().count();
    let mut i=0;
    let mut sp=0;
    while i<lens{
        match code.chars().nth(i).unwrap(){
            '>'=>sp=sp+1,
            '<'=>sp=sp-1,
            '+'=>data[sp]=data[sp]+1,
            '-'=>data[sp]=data[sp]-1,
            '.'=>print!("{}",std::char::from_u32(data[sp]).unwrap()),
            ','=>println!("read"),
            '['=>if data[sp]==0{
                let mut cnt=1;
                while cnt != 0{
                    i=i+1;
                    match code.chars().nth(i).unwrap(){
                        ']'=>cnt=cnt-1,
                        '['=>cnt=cnt+1,
                        _ =>{},
                    }
                }
                continue;
            }
            ']'=> if data[sp]!=0{
                let mut cnt=1;
                while cnt !=0{
                    i=i-1;
                    match code.chars().nth(i).unwrap(){
                        ']'=>cnt=cnt+1,
                        '['=>cnt=cnt-1,
                        _ =>{},
                    }
                }
                continue;
            }
            _=>{},
        }
        i=i+1;
    }
}
