pub fn run () {

}

fn count(v: &Vec<i32>, val: i32) -> usize {
  v.into_iter().filter(|&&x| x == val).count()
}

fn copy<T>(t: T) -> T
where // Guard
  T: Copy, // T must ba able to implement Trait Copy
{
  let x = t;
  x
}