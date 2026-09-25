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

fn robot_debe_terminar(cinta: &Cinta) -> bool {
    cinta.paquetes.is_empty() && cinta.finalizada
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

    let camiones: Vec<_> = (1..=CANTIDAD_CAMIONES)
        .map(|id| iniciar_camion(id, Arc::clone(&cinta)))
        .collect();
    let robots: Vec<_> = (1..=CANTIDAD_ROBOTS)
        .map(|id| iniciar_robot(id, Arc::clone(&cinta)))
        .collect();

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

fn iniciar_camion(id: usize, cinta: Arc<(Mutex<Cinta>, Condvar)>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        for numero in 0..PAQUETES_POR_CAMION {
            let paquete_id = ((id - 1) * PAQUETES_POR_CAMION + numero + 1) as u32;
            let (mutex, condvar) = &*cinta;
            let mut cinta = mutex.lock().unwrap();
            while cinta.paquetes.len() == CAPACIDAD_CINTA {
                cinta = condvar.wait(cinta).unwrap();
            }
            cinta.paquetes.push_back(Paquete::nuevo(paquete_id));
            println!("Camión {}: dejó paquete P_{}", id, paquete_id);
            condvar.notify_all();
            drop(cinta);
            thread::sleep(Duration::from_millis(30));
        }
    })
}

fn iniciar_robot(id: usize, cinta: Arc<(Mutex<Cinta>, Condvar)>) -> thread::JoinHandle<()> {
    thread::spawn(move || loop {
        let paquete = {
            let (mutex, condvar) = &*cinta;
            let mut cinta = mutex.lock().unwrap();
            while cinta.paquetes.is_empty() && !cinta.finalizada {
                cinta = condvar.wait(cinta).unwrap();
            }
            if robot_debe_terminar(&cinta) {
                break;
            }
            let paquete = cinta.paquetes.pop_front().unwrap();
            condvar.notify_all();
            paquete
        };
        println!("Robot {}: tomó paquete P_{}", id, paquete.id);
        thread::sleep(Duration::from_millis(50));
    })
}



#[cfg(test)]
mod tests {
    use super::{Cinta, Paquete};

    #[test]
    fn la_cinta_comienza_vacia_y_no_finalizada() {
        let cinta = Cinta::nueva();

        assert!(cinta.paquetes.is_empty());
        assert!(!cinta.finalizada);
    }

    #[test]
    fn la_cinta_conserva_el_orden_fifo() {
        let mut cinta = Cinta::nueva();
        cinta.paquetes.push_back(Paquete::nuevo(1));
        cinta.paquetes.push_back(Paquete::nuevo(2));

        assert_eq!(cinta.paquetes.pop_front().unwrap().id, 1);
        assert_eq!(cinta.paquetes.pop_front().unwrap().id, 2);
    }

    #[test]
    fn el_robot_no_termina_si_quedan_paquetes() {
        let mut cinta = Cinta::nueva();
        cinta.finalizada = true;
        cinta.paquetes.push_back(Paquete::nuevo(1));

        assert!(!super::robot_debe_terminar(&cinta));
    }

    #[test]
    fn el_robot_termina_solo_con_cinta_vacia_y_finalizada() {
        let mut cinta = Cinta::nueva();

        assert!(!super::robot_debe_terminar(&cinta));
        cinta.finalizada = true;
        assert!(super::robot_debe_terminar(&cinta));
    }

}