use inflector::cases::sentencecase::to_sentence_case;
use rand::prelude::*;

pub const PREFIX: &[&str] = &[
  "", "aga", "allo", "an", "andro", "anti", "ap", "aqui", "ar", "ara", "ba", "bacty", "bal", "baltara", "bali", "bara",
  "bas", "bona", "cael", "can", "cap", "cas", "cella", "cen", "cepha", "cer", "ceta", "char", "cir", "co", "coil",
  "col", "cor", "cra", "da", "del", "dor", "dra", "elda", "erra", "esse", "equi", "fel", "for", "fre", "ge", "glo",
  "hal", "hel", "her", "horo", "hy", "hybore", "inno", "jan", "kar", "leo", "libo", "machre", "male", "mono", "na",
  "nigi", "nocto", "nucre", "octa", "ori", "pega", "per", "petra", "pro", "psalta", "pura", "pyxa", "reti", "sagi",
  "scepta", "scoro", "ser", "sol", "speci", "specu", "tar", "tau", "tele", "tria", "ty", "tza", "uralo", "ur", "val",
  "vir", "xer", "yetra",
];
pub const INFIX: &[&str] = &[
  "", "a", "alla", "arra", "attra", "bas", "bella", "beta", "boro", "calla", "cla", "cly", "cres", "cul", "cylandr",
  "cylor", "dyro", "dro", "elter", "escher", "esther", "fenne", "for", "fra", "gelt", "got", "hela", "hera", "irra",
  "is", "kry", "logi", "mely", "mekry", "mes", "met", "metro", "min", "nat", "net", "pat", "pet", "pret", "riota",
  "ser", "sus", "syr", "tar", "tem", "ter", "tin", "tli", "tol",
];
pub const SUFFIX: &[&str] = &[
  "", "a", "an", "aura", "aurus", "eda", "elius", "eon", "eontia", "ese", "eti", "etius", "i", "ion", "lon", "meda",
  "neda", "nia", "olia", "oloria", "on", "ontia", "opia", "or", "oria", "orious", "ous", "os", "tor", "um", "us", "ux",
  "yria",
];

#[named]
pub fn generate_star_name<R: Rng + ?Sized>(rng: &mut R) -> String {
  let first = PREFIX[rng.gen_range(0..PREFIX.len())];
  let second = INFIX[rng.gen_range(0..INFIX.len())];
  let third = SUFFIX[rng.gen_range(0..SUFFIX.len())];

  to_sentence_case(&format!("{}{}{}", first, second, third))
}

#[cfg(test)]
pub mod test {

  use rand::prelude::*;

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn test_name_star() {
    init();

    let mut rng = thread_rng();

    let name = generate_star_name(&mut rng);

    print_var!(name);
  }
}
