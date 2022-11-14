use std::process::Output;

pub struct Link<T, S>
where 
  T: Node,
  S: Node<Input = T::Output>
{
  node_input: T,
  node_output: S,
}

impl<T, S> Link<T, S> 
where 
  T: Node,
  S: Node<Input = T::Output> 
{
  pub fn new(node_1: T, node_2: S) -> Self {
    Link {
      node_input: node_1,
      node_output: node_2,
    }
  }
}
 
impl <T, S> Node for Link<T, S> 
where
  T: Node,
  S: Node<Input = T::Output>
{
  type Input = T::Input;
  type Output = S::Output;

  fn apply(&mut self, input: Self::Input) -> Result<Self::Output, ()> {
      self.node_output.apply(self.node_input.apply(input).unwrap())
  }
}

pub struct LoopLink<T, S>
where 
  T: Node,
  S: Node<Input = T::Output>
{
  node_input: T,
  node_output: S,
}

impl<T, S> LoopLink<T, S>
where 
  T: Node,
  S: Node<Input = T::Output>
{
  
}

// impl<T, S> Node for LoopLink<T, S>
// where
//   T: Node,
//   S: Node<Input = T::Output>
// {
//   type Input = T::Input;
//   type Output = S::Output;

//   fn apply(&mut self, input: Self::Input) -> Result<Self::Output, ()> {
//     let data = self.node_input.apply(input).unwrap();

//     loop {
//       let result = self.node_output.apply(data);
//       if result.is_err() {
//         return Err(())
//       }
//     }


//     Err(())
//   }
// }

pub trait Node {
  type Input;
  type Output;

  fn apply(&mut self, input: Self::Input) -> Result<Self::Output, ()>;
  fn add_node<T>(self, node: T) -> Link<Self, T>
  where
    T: Node<Input = Self::Output>,
    Self: Sized
  {
    Link::new(self, node)
  }

  fn add_loop<T>(self, node: T) -> LoopLink<Self, T>
  where
    T: Node<Input = Self::Output>,
    Self: Sized
  {
    LoopLink { node_input: self, node_output: node }
  }
}

struct MultiplyBy {
  factor: f64
}

impl Node for MultiplyBy {
  type Input = f64;
  type Output = f64;

  fn apply(&mut self, input: Self::Input) -> Result<Self::Output, ()> {
      Ok(self.factor * input)
  }
}

impl MultiplyBy {
  pub fn new(factor: f64) -> Self {
    MultiplyBy { factor }
  }
}

struct AddOne;

impl Node for AddOne {
  type Input = f64;
  type Output = f64;

  fn apply(&mut self, input: Self::Input) -> Result<Self::Output, ()> {
    if input < 10.0 {
      Ok(input + 1.0)
    } else {
      Err(())
    }
  }
}

impl AddOne {
  pub fn new() -> Self {
    AddOne {}
  }
}

struct Stringify;

impl Stringify {
  pub fn new() -> Self {
    Stringify {}
  }
}

impl Node for Stringify {
  type Input = f64;
  type Output = String;

  fn apply(&mut self, input: Self::Input) -> Result<Self::Output, ()> {
    Ok(input.to_string())
  }
}

pub fn pipeline_test() {
  let mut pipeline = MultiplyBy::new(3.0)
    .add_node(AddOne::new())
    .add_node(Stringify::new())
    .apply(3.0);

    println!("{}", pipeline.unwrap());
}
