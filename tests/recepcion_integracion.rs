use almacen_inteligente::recepcion;

#[test]
fn la_recepcion_finaliza() {
    recepcion::ejecutar();
}

#[test]
fn la_recepcion_no_panic() {
    let resultado = std::panic::catch_unwind(recepcion::ejecutar);

    assert!(resultado.is_ok());
}

#[test]
fn la_recepcion_puede_ejecutarse_varias_veces() {
    recepcion::ejecutar();
    recepcion::ejecutar();
    recepcion::ejecutar();
}

