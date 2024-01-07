// Importación de macros y dependencias necesarias
#[macro_use] extern crate rocket;
use rocket::State;
use rocket::response::{self, Responder, Response};
use rocket::http::ContentType;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use serde_json::Value;
use std::{collections::{HashMap, HashSet}, sync::{Arc, Mutex}, thread, time::Duration};

// Constante que define el intervalo de tiempo de espera en el hilo
const SLEEP_TIME: u64 = 20;

// Estructura para representar el grafo de transacciones
struct TxGraph {
    // HashMap que almacena las relaciones de transacciones: clave es el ID de la transacción padre, valor es un conjunto de IDs de transacciones hijas
    edges: HashMap<String, HashSet<String>>,
}

impl TxGraph {
    // Constructor para crear un nuevo grafo de transacciones vacío
    fn new() -> TxGraph {
        TxGraph {
            edges: HashMap::new(),
        }
    }

    // Función para agregar una relación padre-hijo entre dos transacciones
    fn add_edge(&mut self, parent_id: String, child_id: String) {
        self.edges.entry(parent_id).or_default().insert(child_id);
    }

    // Función para limpiar el grafo de transacciones, eliminando aquellas que ya no están en la mempool
    fn clean_transactions(&mut self, mempool_txs: &HashMap<String, Value>) {
        self.edges.retain(|tx_id, _| mempool_txs.contains_key(tx_id));
    }
}

// Estructura para manejar contenido HTML como respuesta
struct HtmlContent(String);

// Implementación del trait Responder para HtmlContent, permitiendo su uso como respuesta HTTP
impl<'r> Responder<'r, 'static> for HtmlContent {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> response::Result<'static> {
        Response::build()
            .header(ContentType::HTML)
            .sized_body(self.0.len(), std::io::Cursor::new(self.0))
            .ok()
    }
}

// Ruta del servidor web para obtener las transacciones descendientes en formato HTML
#[get("/get_descen_html/<tx_ini>/<tx_end>")]
fn get_descen_html(graph: &State<Arc<Mutex<TxGraph>>>, tx_ini: usize, tx_end: usize) -> HtmlContent {
    let graph = graph.lock().unwrap();
    let mut transactions = String::new();

    // Generando contenido HTML con las transacciones
    transactions.push_str("<h1>Txs de la Mempool</h1>");
    transactions.push_str(&format!("<h3>Mostrando las transacciones desde la Tx:{} hasta la Tx:{}</h3><br>", tx_ini, tx_end));
    transactions.push_str("<style> .tx-padre { color: black; } .tx-hijo { color: green; } .tx-nieto { color: blue; } </style>");

    // Iterando sobre las transacciones para mostrar sus relaciones en HTML
    for (index, (parent_id, children)) in graph.edges.iter().enumerate() {
        if index >= tx_ini && index <= tx_end {
            transactions.push_str(&format!("<p class='tx-padre'>Tx padre: {} </p>", parent_id));
            for child_id in children {
                transactions.push_str(&format!("<p class='tx-hijo'>&nbsp;&nbsp Tx hijo: {:?}</p>", child_id));

                // Mostrando transacciones nieto, si existen
                if let Some(grandchildrens) = graph.edges.get(child_id) {
                    for grandchildren in grandchildrens {
                        transactions.push_str(&format!("<p class='tx-nieto'>&nbsp;&nbsp;&nbsp;&nbsp; Tx nieto: {:?}</p>", grandchildren));
                    }
                }
            }
        }
    }

    // Empaquetando el contenido HTML como una respuesta
    let html_output = format!("<html><body>{}</body></html>", transactions);
    HtmlContent(html_output)
}

// Función principal para lanzar el servidor Rocket
#[launch]
fn rocket() -> _ {
    // Inicializando el grafo de transacciones y su versión compartida entre hilos
    let graph = Arc::new(Mutex::new(TxGraph::new()));
    let graph_clone = Arc::clone(&graph);

    // Creando un hilo para actualizar el grafo periódicamente
    thread::spawn(move || {
        let rpc_url = "http://localhost:8332";
        let rpc_auth = Auth::UserPass("userX".to_string(), "wsx".to_string());
        let client = Client::new(rpc_url, rpc_auth).expect("Error al conectar con el nodo Bitcoin Core");

        // Bucle infinito para actualizar el grafo
        loop {
            let mempool_txs = get_raw_mempool(&client).expect("Error al obtener transacciones del mempool");
            get_descendants(&mempool_txs, &client, &graph_clone);
            graph_clone.lock().unwrap().clean_transactions(&mempool_txs);

            println!("=> Txs grafo: {}", graph_clone.lock().unwrap().edges.len());
            thread::sleep(Duration::from_secs(SLEEP_TIME));
        }
    });

    // Configurando el servidor Rocket con la ruta definida
    rocket::build().manage(graph).mount("/", routes![get_descen_html])
}

// Función para obtener las transacciones en la mempool del nodo Bitcoin Core
fn get_raw_mempool(client: &Client) -> bitcoincore_rpc::Result<HashMap<String, Value>> {
    match client.call("getrawmempool", &[Value::Bool(true)]) {
        Ok(mempool) => Ok(mempool),
        Err(e) => {
            println!("Error al obtener el mempool: {}", e);
            Err(e)
        }
    }
}

// Función para obtener y procesar los descendientes de las transacciones en la mempool
fn get_descendants(mempool_txs: &HashMap<String, Value>, client: &Client, graph: &Arc<Mutex<TxGraph>>) {
    let mut graph = graph.lock().unwrap();
    let mut num_txs = 0;

    // Iterando sobre las transacciones de la mempool
    for (hash_tx, tx_data) in mempool_txs {
        // Continuar si la transacción ya está en el grafo
        if graph.edges.contains_key(hash_tx) {
            continue;
        } 

        // Procesar solo las transacciones con descendientes
        if let Some(num_desc) = tx_data.get("descendantcount").and_then(Value::as_i64) {
            if num_desc > 0 {
                num_txs += 1;
                if num_txs > 1000 {
                    break;
                }

                // Obtener los descendientes de la transacción actual
                let descendants = get_mempool_descendants(client, hash_tx).unwrap_or_else(|_| vec![]);
                for desc_tx in descendants {
                    graph.add_edge(hash_tx.clone(), desc_tx.clone());

                    // Obtener y procesar los descendientes de los descendientes (nietos)
                    let desc_descendants = get_mempool_descendants(client, &desc_tx).unwrap_or_else(|_| vec![]);
                    for desc_desc_tx in desc_descendants {
                        if graph.edges.contains_key(desc_tx.clone().as_str()) {
                            continue;
                        } 
                        graph.add_edge(desc_tx.clone(), desc_desc_tx);
                    }
                } 
            } 
        }
    }
}

// Función para obtener los descendientes de una transacción específica en la mempool
fn get_mempool_descendants(client: &Client, txid: &str) -> bitcoincore_rpc::Result<Vec<String>> {
    match client.call("getmempooldescendants", &[Value::String(txid.to_string())]){
        Ok(descendants) => Ok(descendants),
        Err(e) => {
            println!("Error al obtener descendientes: {}", e);
            Err(e)
        }
    }
}
