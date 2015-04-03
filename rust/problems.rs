#![feature(core)]

fn main() {
    let v: [i32; 4] = [1, 2, 3, 4];

    // problem 1
    assert_eq!(*my_last(&v).unwrap(), 4);

    // problem 2
    assert_eq!(*my_but_last(&v).unwrap(), 3);

    // problem 3
    assert_eq!(*element_at(&v, 2).unwrap(), 2);

    // problem 4
    assert_eq!(my_length(&v), 4);

    // problem 5
    let reversed = my_reverse(&v);
    assert_eq!(*reversed[0], 4);
    assert_eq!(*reversed[3], 1);

    // problem 6
    assert!(!is_palindrome(&[1,2,3]));
    assert!(is_palindrome(&[1, 2, 4, 8, 16, 8, 4, 2, 1]));

    // problem 7
    let nl = NestedList::List(
               vec![NestedList::Elem(1),
               NestedList::List(
                 vec![NestedList::Elem(2),
                      NestedList::List(
                        vec![NestedList::Elem(3),
                             NestedList::Elem(4)]),
                        NestedList::Elem(5)])]);
    assert_eq!(*(flatten(&nl)[0]), 1);
    assert_eq!(*(flatten(&nl)[2]), 3);
    assert_eq!(*(flatten(&nl)[4]), 5);
}


fn my_last<T>(v: &[T]) -> Option<&T> {
    v.last()
}


fn my_but_last<T>(v: &[T]) -> Option<&T> {
    if v.len() > 1 { Some(&v[v.len() - 2]) }
    else { None }
}


fn element_at<T>(v: &[T], i: usize) -> Option<&T> {
    if v.len() > i { Some(&v[i - 1]) }
    else { None }
}


fn my_length<T>(v: &[T]) -> usize {
    v.len()
}


fn my_reverse<T>(v: &[T]) -> Vec<&T> {
  let mut r = Vec::with_capacity(v.len());

  for x in v.iter().rev() {
    r.push(x);
  }
  r
}


fn is_palindrome<T: Eq>(v: &[T]) -> bool {
  for i in 0 .. v.len() / 2 {
    if v[i] != v[v.len() - i - 1] { return false }
  }
  true
}


enum NestedList<T> {
  Elem(T),
  List(Vec<NestedList<T>>)
}

fn flatten<T>(l: &NestedList<T>) -> Vec<&T> {
  match *l {
    NestedList::Elem(ref x) => vec![x],
    NestedList::List(ref v) => v.iter().flat_map(|x| flatten(x).into_iter()).collect()
  }
}

