struct ABig๐ฉโ๐ฉโ๐งโ๐งFamily; //~ ERROR identifiers cannot contain emoji
struct ๐; //~ ERROR identifiers cannot contain emoji
impl ๐ {
    fn full_of_โจ() -> ๐ { //~ ERROR identifiers cannot contain emoji
        ๐
    }
}
fn i_like_to_๐_a_lot() -> ๐ { //~ ERROR identifiers cannot contain emoji
    ๐::full_ofโจ() //~ ERROR no function or associated item named `full_ofโจ` found for struct `๐`
    //~^ ERROR identifiers cannot contain emoji
}
fn main() {
    let _ = i_like_to_๐_a_lot() โ 4; //~ ERROR cannot find function `i_like_to_๐_a_lot` in this scope
    //~^ ERROR identifiers cannot contain emoji
    //~| ERROR unknown start of token: \u{2796}

    let ๐ฆ = 1;//~ ERROR Ferris cannot be used as an identifier
    dbg!(๐ฆ);
}
