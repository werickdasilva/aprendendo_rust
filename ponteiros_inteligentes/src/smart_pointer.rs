pub fn smart_pointer_box() {
    let box_tipo = Box::new(100);
    println!("Valor da Box é {}",box_tipo);

    let box_em_trait = BoxEmTrait {
        contrato: Box::new(ImplementaContrato{})
    };

    box_em_trait.contrato.mostrar();
}


// box em trait
struct BoxEmTrait {
    contrato: Box<dyn Contrato>
}

struct ImplementaContrato;

impl Contrato for ImplementaContrato {
    fn mostrar(&self) {
        println!("Implementado o contrato")
    }
}

trait Contrato {
    fn mostrar(&self);
}