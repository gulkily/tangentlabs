/// example of binary file io
use std::fs::File;
use std::io::prelude::*;

struct Data { a: u8, b: u8, c: u16, d: u32 }

// https://stackoverflow.com/questions/28127165/how-to-convert-struct-to-u8
unsafe fn to_u8s<T: Sized>(p: &T) -> &[u8] {
  ::std::slice::from_raw_parts(
    (p as *const T) as *const u8,
    ::std::mem::size_of::<T>()) }

// adapted from the above, to deal with a slice:
unsafe fn slice_to_u8s<T: Sized>(p: &[T]) -> &[u8] {
  ::std::slice::from_raw_parts(
    (p.as_ptr()) as *const u8,
    ::std::mem::size_of::<T>() * p.len()) }

unsafe fn u8s_to_slice<T: Sized>(p: &[u8]) -> &[T] {
  ::std::slice::from_raw_parts(
    (p.as_ptr()) as *const T,
    p.len() / ::std::mem::size_of::<T>()) }

impl Data {
  fn new(n:u8) -> Data { Data{a:n, b:n, c:n as u16, d: n as u32 }} }

fn do_main()->std::io::Result<()> {
  let mut m = vec![];
  for i in 0..255 { m.push(Data::new(i)) }

  // let bytes = unsafe { to_slice(&m) };

  let path = "binfile.dat";

  { // write
    let mut f = File::create(path)?;

    // first write each record using a loop:
    for (_,d) in m.iter().enumerate() { f.write_all(unsafe { to_u8s(d) } )? }

    // then write a second copy, storing entire vector at once:
    f.write_all( unsafe{ slice_to_u8s(m.as_slice()) })?; }

  let mut u = Vec::new();
  let r: &[Data] = { // read
    let mut f = File::open(path)?;
    // TODO: read_exact
    f.read_to_end(&mut u).expect("couldn't read file");
    let s = u.as_slice();
    unsafe { u8s_to_slice(s)} };

  Ok(()) }

fn main() {
  do_main().expect("couldn't do it."); }
