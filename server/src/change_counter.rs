use std::sync::Mutex;

pub struct ChangeCounter {
  pub counter : Mutex<i32>
}