#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]
//! Integration tests
{% if async -%} 
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

static INIT: std::sync::Once = std::sync::Once::new();
fn setup_test_tracing() {
  INIT.call_once(|| {
    let subscriber =
      FmtSubscriber::builder().with_max_level(Level::INFO).with_test_writer().finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
  });
}
{% endif -%}
use arbitrary::Arbitrary;
use rstest::{fixture, rstest};
// rstest provides features to take common context into tests, and set up small cases testing
#[derive(Clone, Debug, Eq, PartialEq, Arbitrary)]
struct Wb {
  b:     bool,
  count: usize,
}
// context setup function to be implicitly called by `wb`
#[fixture]
fn count() -> usize { return 0usize; }
// context setup function to be implicitly called by `test_wb`
#[fixture]
fn wb(#[default(false)] b: bool, count: usize) -> Wb {
  {% if async -%} setup_test_tracing(); {% else -%} 
  let _ = env_logger::builder().filter_level(log::LevelFilter::Debug).is_test(true).try_init();
  {% endif -%}
  Wb { b, count }
}

// small-cases fuzzing
// argument wb will inherit the above function if names match; will generate 3x3 case-tests
#[rstest]
#[case(0, true, true)]
#[case(1, true, false)]
fn test_wb(wb: Wb, #[case] n: usize, #[case] b: bool, #[case] expected: bool) {
  {% if async %} tracing::info!("wb: {wb:?}"); {% else %} log::info!("wb: {wb:?}"); {% endif %}
  let wb_ = Wb { count: n, b };
  assert_eq!(wb == wb_, expected); // this will fail for case_1 cases
}

// ex 2 - baby fuzz; will generate 2x2 test cases
#[rstest]
fn test_enumerative(#[values(0, 4)] n: usize, #[values(7, 8)] m: usize) {
  assert!(n < m);
}

// fuzz test
fn reverse<T: Clone>(xs: &[T]) -> Vec<T> {
  let mut rev = vec![];
  for x in xs.iter() {
    rev.insert(0, x.clone())
  }
  rev
}

// fuzz, declare quickcheck on any argument implementing Arbitrary
#[quickcheck_macros::quickcheck]
fn prop(xs: Vec<u32>) -> bool { xs == reverse(&reverse(&xs)) }
