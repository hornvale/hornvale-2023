use regex::Regex;

use super::*;

impl ProcessOutput {
  /// Format output string.
  pub fn format_string(&mut self, string: &str) -> String {
    lazy_static! {
      static ref FOREGROUND: Regex = Regex::new(r"<fg_ext(\d+)>").unwrap();
      static ref BACKGROUND: Regex = Regex::new(r"<bg_ext(\d+)>").unwrap();
    }
    let mut result = string.to_owned();
    result = FOREGROUND.replace_all(&result, "\x1B[38;5;${1}m").to_string();
    result = BACKGROUND.replace_all(&result, "\x1B[48;5;${1}m").to_string();
    result = result
      .replace("<reset>", "\x1B[0m")
      .replace("<black>", "\x1B[30m")
      .replace("<red>", "\x1B[31m")
      .replace("<green>", "\x1B[32m")
      .replace("<yellow>", "\x1B[33m")
      .replace("<blue>", "\x1B[34m")
      .replace("<magenta>", "\x1B[35m")
      .replace("<cyan>", "\x1B[36m")
      .replace("<white>", "\x1B[37m")
      .replace("<default>", "\x1B[39m")
      .replace("<bg_black>", "\x1B[40m")
      .replace("<bg_red>", "\x1B[41m")
      .replace("<bg_green>", "\x1B[42m")
      .replace("<bg_yellow>", "\x1B[43m")
      .replace("<bg_blue>", "\x1B[44m")
      .replace("<bg_magenta>", "\x1B[45m")
      .replace("<bg_cyan>", "\x1B[46m")
      .replace("<bg_white>", "\x1B[47m")
      .replace("<bg_default>", "\x1B[49m")
      .replace("<br_black>", "\x1B[90m")
      .replace("<br_red>", "\x1B[91m")
      .replace("<br_green>", "\x1B[92m")
      .replace("<br_yellow>", "\x1B[93m")
      .replace("<br_blue>", "\x1B[94m")
      .replace("<br_magenta>", "\x1B[95m")
      .replace("<br_cyan>", "\x1B[96m")
      .replace("<br_white>", "\x1B[97m")
      .replace("<bg_br_black>", "\x1B[100m")
      .replace("<bg_br_red>", "\x1B[101m")
      .replace("<bg_br_green>", "\x1B[102m")
      .replace("<bg_br_yellow>", "\x1B[103m")
      .replace("<bg_br_blue>", "\x1B[104m")
      .replace("<bg_br_magenta>", "\x1B[105m")
      .replace("<bg_br_cyan>", "\x1B[106m")
      .replace("<bg_br_white>", "\x1B[107m")
      .replace("<bold>", "\x1B[1m")
      .replace("<dim>", "\x1B[2m")
      .replace("<italic>", "\x1B[3m")
      .replace("<underline>", "\x1B[4m")
      .replace("<blink>", "\x1B[5m")
      .replace("<reverse>", "\x1B[7m")
      .replace("<hidden>", "\x1B[8m")
      .replace("<strikethrough>", "\x1B[9m")
      .replace("</bold>", "\x1B[21m")
      .replace("</dim>", "\x1B[22m")
      .replace("</italic>", "\x1B[23m")
      .replace("</underline>", "\x1B[24m")
      .replace("</blink>", "\x1B[25m")
      .replace("</reverse>", "\x1B[27m")
      .replace("</hidden>", "\x1B[28m")
      .replace("</strikethrough>", "\x1B[29m");

    format!("{}\x1B[0m", result)
  }
}
