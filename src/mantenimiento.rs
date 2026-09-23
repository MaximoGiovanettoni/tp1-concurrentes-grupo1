use std::sync::{
    mpsc,
    Arc,
    Mutex
};
use std::thread;
use std::time::Duration;

struct SolicitudMantenimiento {
    robot_id: usize,
}

struct EstadoMantenimiento {
    mecanico_ocupado: bool,
    bahias_ocupadas: usize,
}

const CANTIDAD_BAHIAS: usize = 4;
const CANTIDAD_ROBOTS_MANTENIMIENTO: usize = 8;

pub fn ejecutar() {
    let estado = Arc::new(Mutex::new(EstadoMantenimiento { mecanico_ocupado: false, bahias_ocupadas: 0 }));
    let (emisor, receptor) = mpsc::channel::<SolicitudMantenimiento>();
    let estado_mecanico = Arc::clone(&estado);
    let mecanico = thread::spawn(move || atender_solicitudes(receptor, estado_mecanico));
    let mut robots = Vec::new();

    for robot_id in 1..=CANTIDAD_ROBOTS_MANTENIMIENTO {
        let estado = Arc::clone(&estado);
        let emisor = emisor.clone();
        robots.push(thread::spawn(move || iniciar_mantenimiento(robot_id, estado, emisor)));
    }
    drop(emisor);
    robots.into_iter().for_each(|robot| robot.join().unwrap());
    mecanico.join().unwrap();
    println!("Mantenimiento finalizado.");
}

fn atender_solicitudes(receptor: mpsc::Receiver<SolicitudMantenimiento>, estado: Arc<Mutex<EstadoMantenimiento>>) {
    while let Ok(solicitud) = receptor.recv() {
        println!("Mecánico: atendiendo al Robot {}", solicitud.robot_id);
        thread::sleep(Duration::from_millis(100));
        let mut estado = estado.lock().unwrap();
        estado.bahias_ocupadas = estado.bahias_ocupadas.saturating_sub(1);
        if estado.bahias_ocupadas == 0 { estado.mecanico_ocupado = false; }
        println!("Mecánico: terminó con el Robot {}", solicitud.robot_id);
    }
    println!("Mecánico: no quedan solicitudes.");
}

fn iniciar_mantenimiento(robot_id: usize, estado: Arc<Mutex<EstadoMantenimiento>>, emisor: mpsc::Sender<SolicitudMantenimiento>) {
    let puede_ser_atendido = {
        let mut estado = estado.lock().unwrap();
        if !estado.mecanico_ocupado {
            estado.mecanico_ocupado = true;
            true
        } else if estado.bahias_ocupadas < CANTIDAD_BAHIAS {
            estado.bahias_ocupadas += 1;
            true
        } else { false }
    };
    if puede_ser_atendido {
        println!("Robot {}: espera para mantenimiento.", robot_id);
        emisor.send(SolicitudMantenimiento { robot_id }).unwrap();
    } else { println!("Robot {} rechazado, vuelve a trabajar.", robot_id); }
}