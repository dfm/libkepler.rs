use libkepler_proc_macro::expand_to_order;

fn eval_at_order<const ORDER: usize>() -> usize {
    ORDER + 1
}

macro_rules! do_order_sum {
    ( $x0:expr ) => {
        $x0
    };
    ( $x0:expr, $( $x:expr ),+ ) => {
        $x0 + do_order_sum!( $( $x ),+ )
    };
}

#[test]
fn test_expand_to_order() {
    assert_eq!(expand_to_order!(do_order_sum!, eval_at_order, 5), 15);
}
