fn area1(w: u32, h: u32) -> u32 { w * h }

fn area2(t: (u32, u32)) -> u32 { t.0 * t.1 }

struct R { w: u32, h: u32 }

fn area3(r: &R) -> u32 { r.w * r.h }


fn main() {
  let w = 30;
  let h = 50;
  println!("{}", area1(w, h));

  let t = (40, 50);
  println!("{}", area2(t));

  let r = R { w: 50, h: 50 };
  println!("{}", area3(&r));
}
