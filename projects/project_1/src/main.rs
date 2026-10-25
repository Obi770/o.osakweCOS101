use::std::io::Write;
use::std::io::Read;

fn main(){
    let header = "Lager,Stout,Non-Alcoholic\n";
    let row1 = "33 Export,Legend,Maltina\n";
    let row2 = "Desperados,Turbo King,Amstel Malta\n";
    let row3 = "Golderg,Williams,Malt Gold\n";
    let row4 = "Golder,_,Fayrouz\n";
    let row5 = "Heineken\n";
    let row6 = "Star\n";


    let mut file = std::fs::File::create("Nigerian_Breweries.csv").expect("Failed to create");
    file.write_all(header.as_bytes()).expect("Failed to write");
    file.write_all(row1.as_bytes()).expect("Failed to write");
    file.write_all(row2.as_bytes()).expect("Failed to write");
    file.write_all(row3.as_bytes()).expect("Failed to write");
    file.write_all(row4.as_bytes()).expect("Failed to write");
    file.write_all(row5.as_bytes()).expect("Failed to write");
    file.write_all(row6.as_bytes()).expect("Failed to write");

    let mut file = std::fs::File::open("Nigerian_Breweries.csv").expect("Failed to open");
    let mut drinks = String::new();
    file.read_to_string(&mut drinks).expect("Failed to open");
    println!("{}",drinks );

}