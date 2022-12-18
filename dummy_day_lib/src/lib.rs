use proc_macro::TokenStream;

#[proc_macro]
pub fn dummy_day_mod(item: TokenStream) -> TokenStream {
    let value =
        item.into_iter().next().unwrap_or_else(
            || panic!("The expected syntax for a dummy day is `dummy_day_mod!(day_number)`")
        );
    format!("\
#[cfg(feature = \"dummy-feature\")]
#[path = \"code/day{}.rs\"]
mod day{};
#[cfg(feature = \"dummy-feature\")]
pub fn day{}_dummy_uses() {{
  let data = day{}::convert(day{}::free_convert(Vec::new()), std::time::Instant::now());
  day{}::run(data.clone());
  #[cfg(feature = \"part-two\")]
    day{}::run_step2(data);
}}", value, value, value, value, value, value, value).parse::<TokenStream>().unwrap()

}