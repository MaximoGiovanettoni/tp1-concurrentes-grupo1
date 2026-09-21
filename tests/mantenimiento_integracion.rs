use almacen_inteligente::mantenimiento;

#[test]
fn el_mantenimiento_finaliza() {
    mantenimiento::ejecutar();
}