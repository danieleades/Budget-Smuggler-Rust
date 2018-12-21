use decimal::d128;
use budget_lib::Budget;
use clap::{crate_authors, crate_version, value_t, App, AppSettings, ArgMatches};
use std::str::FromStr;

pub fn run(budget: &mut Budget<d128>) {

}

fn get_app<'a, 'b>() -> App<'a, 'b> {
    App::new("Budget-Smuggler")
        .author(crate_authors!("\n"))
        .version(crate_version!())
        .about(
            r#"
 ___         _          _       ___                     _         
| _ )_  _ __| |__ _ ___| |_ ___/ __|_ __ _  _ __ _ __ _| |___ _ _ 
| _ \ || / _` / _` / -_)  _|___\__ \ '  \ || / _` / _` | / -_) '_|
|___/\_,_\__,_\__, \___|\__|   |___/_|_|_\_,_\__, \__, |_\___|_|  
              |___/                          |___/|___/           


            .'*'*'.  .'```'.
           (..)O))) (..)O)))
           (\/  ))))(\/  ))))
            \ ## ))) \ oo )))\
            |`''`%%/%|`''`""/"\
            |%%%%%/%%|"""""/"""\
            |%%%%%|%%|"""""|""""' 
            `%%%%%%\%`""""""\""""\
             \%%%%%%\%\""""""\\""".
              |%%%%%%\%|""""""\\\\|
 ____________;;%_%;;%_;;"_";;"_\\\|____________
 ____________\'___'\__\'___'\__________________
                  \%%%%\\| \""""\\|
                   `%%%%\\  `""""\\
                     `%%\\    `""\\
                       \#\\     \%\\
                        \#\\     \%\\
                         \#\\     \%\\
                          \#\\     \%\\
                           \#\\     \%\\
                            \#\      \%\
                             \#\      \%\
                              \#\      \%\
                               \#\      \%\
                                \\       \\
                                
        Personal finance and budgeting app."#,
        )
        // app settings
        .setting(AppSettings::AllowNegativeNumbers)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        // subcommands

}

// this trait is a wrapper around clap's 'value_t' macro
pub trait AppTools {
    type Err;
    /// returns a converted value. the type must implement 'FromStr'. the optional format string is added to the user message to explain the correct format for this type.
    fn typed_value_of<T: FromStr>(&self, name: &str, format: Option<&str>) -> Option<T>;
}

impl<'a> AppTools for ArgMatches<'a> {
    type Err = clap::Error;

    fn typed_value_of<T: FromStr>(&self, name: &str, format: Option<&str>) -> Option<T> {
        match value_t!(self.value_of(name), T) {
            Ok(x) => Some(x),
            Err(ref x) if x.kind == clap::ErrorKind::ArgumentNotFound => None,
            Err(mut x) => {
                if let Some(f) = format {
                    x.message += &format!(" [{}]", f);
                }
                x.exit()
            }
        }
    }
}
