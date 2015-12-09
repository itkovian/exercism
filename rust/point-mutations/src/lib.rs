
pub fn hamming_distance<'a>(source: &'a str, target: &str) -> Result<u64, &'a str> {

    let d = source.chars().zip(target.chars())
                  .fold(0, |i, (s, t)| {
                      if s != t {
                          i+1
                      } else {
                          i
                      }
                  });
    Ok(d)
}
