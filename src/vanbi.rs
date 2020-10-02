use clap::ArgMatches;

pub struct Vanbi<'a> {
  tcekau_selcuha: &'a ArgMatches<'a>,
}

impl<'a> Vanbi<'a> {
  pub fn new(selcuha: &'a ArgMatches<'a>) -> Self {
    Vanbi {
      tcekau_selcuha: selcuha,
    }
  }
}
