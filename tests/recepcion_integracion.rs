use almacen_inteligente::recepcion;
use std::thread;

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

#[test]
fn la_recepcion_puede_ejecutarse_en_paralelo() {
    let ejecuciones: Vec<_> = (0..3)
        .map(|_| thread::spawn(recepcion::ejecutar))
        .collect();

    for ejecucion in ejecuciones {
        assert!(ejecucion.join().is_ok());
    }
}

