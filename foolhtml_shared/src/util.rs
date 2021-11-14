/// easily declare a vector of Strings using str literals
///...
/// assert_eq!(vec!["hello".to_string()], string_vec!["hello"])
///...
#[macro_export]
macro_rules! string_vec {
    ( $( $x:expr ),* ) => {
        vec![$($x.to_string(),)*]
    }
}

#[macro_export]
macro_rules! node_el_vec {
    ($($elem:expr),+) => {
        vec![$(Node::ELEM($elem)),+]
    }
}
