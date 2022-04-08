struct Stack<T> {
  stack: Vec<T>
}
impl<T> Stack<T>{
  fn new()-> Self{
    Stack{stack: vec![]}
  }
  fn add(&mut self, str: T){
    self.stack.push(str);
  }
  fn remove(&mut self)-> Option<T>{
    self.stack.pop()
  }
  fn get_size(&mut self) -> usize{
    self.stack.len()
  }
}

fn main(){
  let word = String::from("(Co(d(er)byte))");
  println!("{}",validate_brackets(&word)); 
}
fn validate_brackets(string: &String)-> bool{
  let open: char = '(';
  let close: char = ')';
  let mut result = true;
  let mut st:Stack<i8> = Stack{stack: vec![]};
  
  for s in string.chars(){
    if s==open {
      st.add(1);
    }
    else if s==close {
      let res = st.remove();
      match res {
        Some(num) => println!("pass..."),
        None => result = false
      };
    }
  };
  result &&  st.get_size()==0
}



