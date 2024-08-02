use core::panic;

#[ic_cdk::query]
fn dodaj_liczby(liczba1: i32, liczba2: i32) -> i32 {
    let wynik = liczba1 + liczba2;
    return wynik;
}
#[ic_cdk::query]
fn odejmij_liczby(liczba1: i32, liczba2: i32) -> i32 {
    let wynik = liczba1 - liczba2;
    return wynik;
}
#[ic_cdk::query]
fn pomnoz_liczby(liczba1: i32, liczba2: i32) -> i32 {
    let wynik = liczba1 * liczba2;
    return wynik;
}
#[ic_cdk::query]
fn podziel_liczby(liczba1: i32, liczba2: i32) -> i32 {
    if liczba2 == 0 {
        panic!("NIE MOZNA DZIELIC PRZEZ ZERO")
    }
    let wynik = liczba1 / liczba2;
    return wynik;
}
#[ic_cdk::query]
fn oblicz_delte(liczba_a: i32, liczba_b: i32, liczba_c: i32) -> i32 {
    let wynik = (liczba_b * liczba_b) - 4 * liczba_a * liczba_c;
    return wynik;
}
#[ic_cdk::query]
fn oblicz_pierwiastek_kwadratowy(liczba: f64) -> f64{
    if liczba >= 0.0{
        let wynik = liczba.sqrt();
        return wynik;
    }
    else{
        panic!("PIERWIASTKOWANA LICZBA MUSI BYC WIEKSZA OD ZERA");
    }
}
#[ic_cdk::query]
fn oblicz_pierwiastek_szescienny(liczba: f64) -> f64{
    let wynik = liczba.cbrt();
    return wynik;
}