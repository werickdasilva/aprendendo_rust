fn main() {
    let people = People;
    let bank = Bank;

    show_info(&bank);


    // mais de um trait em tipo generico
    show_info_e_to_string(&people);

    let tipo_generico_em_struct = TipoGenerico {
        info: people,
    };

    tipo_generico_em_struct.mosta_resulado();
}


trait Info {
    fn show(&self);

    fn is_state(&self) {

    }
}

struct People;

struct Bank;

impl Info for People {
    fn show(&self){
        println!("trait Info metodo show da struct People")
    }
}

impl ToString for People {
    fn to_string(&self) -> String {
        "People foi chamado ".to_string()
    }
}
impl Info for Bank {
    fn show(&self){
        println!("trait Info metodo show da struct Bank")
    }
}

// trait bounds
fn show_info<I: Info>(info: &I) {
    info.show()
}

fn show_info_e_to_string<I>(info: &I) where I: Info + ToString{
    println!("+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+");
    info.show();
    println!("{}", info.to_string());
    println!("+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+");
}
struct TipoGenerico<T> {
    info: T,
}

impl<T: Info> TipoGenerico<T> {
    fn mosta_resulado(&self) {
        println!("========= Resultado ==============");
        self.info.show();
        println!("========= ========= ==============");
    }
}