# tp1-concurrentes-grupo1

## Plan de pruebas: parte 4 - zona de recepción

### Objetivo

Verificar que la simulación de recepción:

- procesa los 20 paquetes producidos por los 2 camiones;
- coordina correctamente a los camiones y a los robots mediante la cinta
	compartida;
- no queda bloqueada cuando la cinta está llena o vacía;
- permite que todos los hilos finalicen y que `recepcion::ejecutar()` retorne;
- puede ejecutarse más de una vez sin dejar estado compartido ni hilos de una
	ejecución anterior.

### Cómo se verificó la ausencia de deadlocks

La verificación se realizó combinando una inspección del protocolo de
sincronización con ejecuciones automatizadas repetidas. El estado de la cinta
(`VecDeque` y la bandera `finalizada`) está protegido por un único `Mutex`, y
el `Condvar` siempre se utiliza asociado a ese mutex.

1. **Cinta llena:** cada camión espera dentro de un `while` mientras la cinta
	 tiene 10 paquetes. `condvar.wait(cinta)` libera el mutex durante la espera,
	 por lo que los robots pueden adquirirlo, retirar paquetes y notificar a los
	 camiones. Cuando hay espacio, el camión continúa.
2. **Cinta vacía:** cada robot espera mientras no hay paquetes y la recepción
	 todavía no terminó. La espera también libera el mutex, de modo que los
	 camiones pueden producir y notificar a los robots.
3. **Finalización:** el hilo principal hace `join` de los dos camiones antes
	 de marcar la cinta como `finalizada`. Así se garantiza que no quedan
	 productores que puedan agregar paquetes. Después cambia la bandera bajo el
	 mutex y ejecuta `notify_all()`, despertando a todos los robots que todavía
	 estén esperando.
4. **Salida de los robots:** al despertarse, un robot termina únicamente cuando
	 la cinta está vacía y `finalizada` es verdadera. Si todavía hay paquetes,
	 los procesa antes de salir.
5. **Ausencia de espera circular:** los hilos nunca conservan el mutex durante
	 el procesamiento de un paquete ni durante el `sleep`. Los únicos bloqueos
	 son esperas sobre la condición de la cinta, y esas esperas liberan el mutex.
	 Por lo tanto, un camión bloqueado depende de que un robot retire paquetes,
	 y un robot bloqueado depende de que un camión agregue paquetes, pero ninguno
	 impide que el otro avance. Finalmente, los `join` se ejecutan en un orden
	 compatible con esas condiciones de salida.

Esta inspección permite descartar el ciclo de espera que produciría un
deadlock. Las pruebas de ejecución complementan el análisis: si alguno de los
`join` no retornara, la prueba quedaría bloqueada y no finalizaría.

### Casos de prueba

Las pruebas de integración se encuentran en
`tests/recepcion_integracion.rs`:

- **`la_recepcion_finaliza`:** ejecuta una recepción completa. El hecho de que
	la función retorne demuestra que los camiones y los robots pudieron terminar
	y que todos sus `join` finalizaron.
- **`la_recepcion_no_panic`:** ejecuta la recepción dentro de
	`catch_unwind` y verifica que el resultado sea `Ok`. Esto detecta fallos de
	sincronización como un `unwrap` sobre un mutex envenenado o un hilo que
	termina inesperadamente.
- **`la_recepcion_puede_ejecutarse_varias_veces`:** ejecuta la recepción tres
	veces consecutivas. Comprueba que cada invocación crea y libera correctamente
	sus propios hilos y su propia cinta, sin depender de estado global residual.
- **`la_recepcion_puede_ejecutarse_en_paralelo`:** inicia tres recepciones al
	 mismo tiempo y espera sus `join`. Comprueba que las cintas son independientes
	 y que la sincronización de una ejecución no bloquea a las otras.

Los tests unitarios de `src/recepcion.rs` cubren casos borde de la estructura
compartida: cinta inicialmente vacía, conservación del orden FIFO, robot con
paquetes pendientes y robot que solo termina cuando la cinta está vacía y fue
marcada como finalizada.

### Ejecución del plan

Se ejecutó la suite completa en serie para hacer más reproducible la salida:

```bash
cargo test -- --test-threads=1
```

Resultado obtenido:

- pruebas unitarias: `4 passed`;
- integración de mantenimiento: `3 passed`;
- integración de recepción: `4 passed`;
- pruebas de documentación: `0 passed, 0 failed`;
- total: `0 failed`.

En particular, las cuatro pruebas de recepción finalizaron correctamente en
aproximadamente 2,17 segundos. Esto confirma experimentalmente que no se
observó un deadlock en la ejecución completa, que la señal de finalización
despierta a los robots y que `recepcion::ejecutar()` retorna normalmente.
 
