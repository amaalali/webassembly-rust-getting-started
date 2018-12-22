extern {
  fn writeDataToNode(x: u32);
}

#[no_mangle]
pub extern "C" fn runJsFn() {
  unsafe {
    writeDataToNode(42)
  }
}
