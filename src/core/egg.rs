pub struct Egg {
   name: String,
   preinstall: Option<Vec<String>>,
   install: Option<Vec<String>>,
   update: Option<Vec<String>>,
   uninstall: Option<Vec<String>>,
}
