use getset::Getters;
use sable_hir::definition::OwnerId;

use crate::package::Package;

#[derive(Getters, Debug)]
pub struct GlobalContext {
  packages: Vec<Package>,
}

impl GlobalContext {
  pub fn new() -> Self {
    Self {
      packages: Vec::new(),
    }
  }

  pub fn create_package(&mut self) -> &Package {
    let def = OwnerId(self.packages.len());
    let package = Package::new(def);
    self.packages.push(package);
    self.packages.last().expect("Package should be created")
  }
}
