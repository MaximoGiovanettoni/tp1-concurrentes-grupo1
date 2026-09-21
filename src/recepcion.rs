use std::sync::{
    Arc,
    Condvar,
    Mutex
};
use std::collections::VecDeque;
use std::thread;
use std::time::Duration;
const CAPACIDAD_CINTA: usize = 10;
const CANTIDAD_CAMIONES: usize = 2;
const PAQUETES_POR_CAMION: usize = 10;
const CANTIDAD_ROBOTS: usize = 3;

#[derive(Debug, Clone)]
struct Paquete {
    id: u32,
}

impl Paquete {
    fn nuevo(id: u32) -> Self {
        Self { id }
    }
}

struct Cinta {
    paquetes: VecDeque<Paquete>,
    finalizada: bool,
}

impl Cinta {
    fn nueva() -> Self {
        Self {
            paquetes: VecDeque::with_capacity(CAPACIDAD_CINTA),
            finalizada: false,
        }
    }
}


pub fn ejecutar() {
    let cinta = Arc::new((
        Mutex::new(Cinta::nueva()),
        Condvar::new(),
    ));

    let mut camiones = Vec::new();

    for camion_id in 1..=CANTIDAD_CAMIONES {
        let cinta_compartida = Arc::clone(&cinta);

        let camion = thread::spawn(move || {
            for numero_paquete in 0..PAQUETES_POR_CAMION {
                let paquete_id =
                    ((camion_id - 1) * PAQUETES_POR_CAMION + numero_paquete + 1) as u32;

                let (mutex, condvar) = &*cinta_compartida;
                let mut cinta = mutex.lock().unwrap();

                while cinta.paquetes.len() == CAPACIDAD_CINTA {
                    cinta = condvar.wait(cinta).unwrap();
                }

                cinta.paquetes.push_back(Paquete::nuevo(paquete_id));

                println!(
                    "Camión {}: dejó paquete P_{}",
                    camion_id, paquete_id
                );

                condvar.notify_all();
                drop(cinta);

                thread::sleep(Duration::from_millis(30));
            }
        });

        camiones.push(camion);
    }

    let mut robots = Vec::new();

    for robot_id in 1..=CANTIDAD_ROBOTS {
        let cinta_compartida = Arc::clone(&cinta);

        let robot = thread::spawn(move || loop {
            let paquete = {
                let (mutex, condvar) = &*cinta_compartida;
                let mut cinta = mutex.lock().unwrap();

                while cinta.paquetes.is_empty() && !cinta.finalizada {
                    cinta = condvar.wait(cinta).unwrap();
                }

                if cinta.paquetes.is_empty() && cinta.finalizada {
                    break;
                }

                let paquete = cinta.paquetes.pop_front().unwrap();
                condvar.notify_all();
                paquete
            };

            println!(
                "Robot {}: tomó paquete P_{}",
                robot_id, paquete.id
            );

            thread::sleep(Duration::from_millis(50));
        });

        robots.push(robot);
    }

    for camion in camiones {
        camion.join().unwrap();
    }

    {
        let (mutex, condvar) = &*cinta;
        let mut cinta = mutex.lock().unwrap();
        cinta.finalizada = true;
        condvar.notify_all();
    }

    for robot in robots {
        robot.join().unwrap();
    }

    println!("Recepción finalizada.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genera_ids_consecutivos() {
        let ids: Vec<u32> = (1..=20).collect();
        assert_eq!(ids.len(), 20);
        assert_eq!(ids.first(), Some(&1));
        assert_eq!(ids.last(), Some(&20));
    }
}