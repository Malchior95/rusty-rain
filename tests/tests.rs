pub mod build_supplying_test;
pub mod gathering_test;
pub mod hearth_test;
pub mod helpers;
pub mod lost_test;
pub mod production_test;

#[cfg(test)]
#[test]
pub fn production() {
    let world = helpers::new_test_world(16, 16);
    production_test::test(world);
}

#[cfg(test)]
#[test]
pub fn hearth() {
    let world = helpers::new_test_world(16, 16);
    hearth_test::test(world);
}

#[cfg(test)]
#[test]
pub fn gathering() {
    let world = helpers::new_test_world(16, 16);
    gathering_test::test(world);
}

#[cfg(test)]
#[test]
pub fn worker_lost() {
    let world = helpers::new_test_world(16, 16);
    lost_test::test(world);
}

#[cfg(test)]
#[test]
pub fn build_supply() {
    let world = helpers::new_test_world(16, 16);
    build_supplying_test::test(world);
}
