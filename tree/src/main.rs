use std::fmt::Debug;
use std::collections::VecDeque;

#[derive(Debug)]
struct NodeInfo<'a,T: 'a + Debug> {
    depth: u32,
    iter: &'a Tree<T>,
}

impl<'a,T: 'a + Debug> NodeInfo<'a,T> {
    fn new(t: u32, it: &'a Tree<T>) -> Self { NodeInfo { depth: t, iter: it, } }
}

#[derive(Debug)]
struct Tree<T:Debug> {
  data: T,
  children: Vec<Tree<T>>,
}

impl<T:Debug> Tree<T> {
  fn new(t: T, c: Vec<Tree<T>>) -> Self { Tree { data: t, children: c, } }
  fn leaf(t: T) -> Self { Tree { data: t, children: vec![], } }
}

// Note that path elements are listed backwards (right-to-left) because we pop
// them off the end.
fn seek_path<T:Debug>(t: &Tree<T>, mut path: Vec<usize>) -> &Tree<T> {
  if let Some(index) = path.pop() {
    seek_path(&t.children[index], path)
  } else {
    t
  }
}

fn print_tree<T:Debug>(t: &Tree<T>) {
    let mut q: VecDeque<NodeInfo<T>> = VecDeque::new();
    let mut depth_old: u32 = 0;
    
    q.push_back(NodeInfo::new(depth_old, t));
    
    while let Some(mut u) = q.pop_front() {
        if u.depth > depth_old {
            println!("{:?}", u.iter.data);
            print!(" ");
        }
        else {
            print!("{:?}", u.iter.data);
            print!(" ");
        }
        
        depth_old = u.depth;
        u.depth += 1;
        
        for kid in u.iter.children.iter() {
            q.push_back(NodeInfo::new(u.depth, kid));
        }
    }
    
    println!("");
}

fn main() {
    let t = Tree::new("a", vec![Tree::leaf("b"),
                                Tree::leaf("c"),
                                Tree::new("d", vec![Tree::leaf("e")])]);
    print_tree(&t);
    
    println!["{:?}", seek_path(&t, vec![0usize])];
    println!["{:?}", seek_path(&t, vec![2usize])];
    println!["{:?}", seek_path(&t, vec![0usize, 2usize])];
}
