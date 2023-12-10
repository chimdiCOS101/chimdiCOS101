fn main() {
    use std::io::Write;
let mut file = std::fs::File::create("Nigerian Breweries plc.txt")
.expect("create failed");

let lager = vec!["\n33 exeport     \tlegend        \tmaltina","\ndesperados  \tTurbo king   \tAmstel malta","\nGoldberg   \twilliams     \tMalta gold","\nGulder        \t-        \tfayrouz", "\nheineken       \t-        \t-","\nstar         \t-            \t-"];




file.write_all("Lager       stout        non alchoholic".as_bytes())
.expect("Failed to write");

for c  in 0..6{
    file.write_all(lager[c].as_bytes())
    .expect("Failed to write");

}




}