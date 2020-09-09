use uno::*;

fn require_num<T: en::Num>() {}

#[test]
fn unorm8_num() {
    require_num::<Unorm8>();
}

#[test]
fn unorm16_num() {
    require_num::<Unorm16>();
}

#[test]
fn unorm32_num() {
    require_num::<Unorm32>();
}

#[test]
fn unorm64_num() {
    require_num::<Unorm64>();
}

#[test]
fn unormsize_num() {
    require_num::<UnormSize>();
}
