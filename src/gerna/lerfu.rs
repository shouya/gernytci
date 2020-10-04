
pub const CURMI_ZUNSNA_REMEI: &'static [(char, char)] = &[
  ('p', 'l'),
  ('p', 'r'),
  ('f', 'l'),
  ('f', 'r'),
  ('b', 'l'),
  ('b', 'r'),
  ('v', 'l'),
  ('v', 'r'),
  ('c', 'p'),
  ('c', 'f'),
  ('c', 't'),
  ('c', 'k'),
  ('c', 'm'),
  ('c', 'n'),
  ('c', 'l'),
  ('c', 'r'),
  ('j', 'b'),
  ('j', 'v'),
  ('j', 'd'),
  ('j', 'g'),
  ('j', 'm'),
  ('s', 'p'),
  ('s', 'f'),
  ('s', 't'),
  ('s', 'k'),
  ('s', 'm'),
  ('s', 'n'),
  ('s', 'l'),
  ('s', 'r'),
  ('z', 'b'),
  ('z', 'v'),
  ('z', 'd'),
  ('z', 'g'),
  ('z', 'm'),
  ('t', 'c'),
  ('t', 'r'),
  ('t', 's'),
  ('k', 'l'),
  ('k', 'r'),
  ('d', 'j'),
  ('d', 'r'),
  ('d', 'z'),
  ('g', 'l'),
  ('g', 'r'),
  ('m', 'l'),
  ('m', 'r'),
  ('x', 'l'),
  ('x', 'r'),
];

pub fn lerpoi_sanse(valsi: &str) -> String {
  let mut teryruhe = String::new();
  for lerfu in valsi.to_lowercase().chars() {
    if let Some(_) = "aeiou".find(lerfu) {
      teryruhe.push('V');
      continue;
    }
    if let Some(_) = "bcdfgjklmnprstvxz".find(lerfu) {
      teryruhe.push('C');
      continue;
    }
    teryruhe.push(lerfu);
  }
  teryruhe
}
