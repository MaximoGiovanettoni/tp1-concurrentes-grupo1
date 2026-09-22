use almacen_inteligente::mantenimiento;

#[test]
fn el_mantenimiento_finaliza() {
    mantenimiento::ejecutar();
}

#[test]
fn el_mantenimiento_no_panic() {
    let resultado = std::panic::catch_unwind(mantenimiento::ejecutar);

    assert!(resultado.is_ok());
}

#[test]
fn el_mantenimiento_puede_ejecutarse_varias_veces() {
    mantenimiento::ejecutar();
    mantenimiento::ejecutar();
    mantenimiento::ejecutar();
}

