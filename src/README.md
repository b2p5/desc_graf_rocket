# Explicación de las partes mas importantes del programa. 



## Estructura de Datos: TxGraph

`TxGraph` es una estructura de datos diseñada para representar un grafo de transacciones en la blockchain de Bitcoin, enfocada en el seguimiento de las relaciones entre las transacciones en la mempool.

## Componentes de TxGraph

### HashMap para Relaciones de Transacciones

- **edges**: `HashMap<String, HashSet<String>>`
  - Cada entrada en este `HashMap` representa una transacción.
  - La clave es el identificador de la transacción padre (`String`).
  - El valor es un `HashSet` de identificadores de transacciones hijas.
  - Este diseño permite un acceso rápido para verificar y enumerar transacciones hijas.

## Funcionalidades de TxGraph

### Constructor `new`

- Crea una instancia vacía de `TxGraph`.
- Inicializa `edges` como un `HashMap` vacío.

### Método `add_edge`

- Añade una relación padre-hijo entre dos transacciones.
- `parent_id`: Identificador de la transacción padre.
- `child_id`: Identificador de la transacción hija.
- Actualiza `edges` adecuadamente.

### Método `clean_transactions`

- Conserva solo las transacciones presentes en la mempool.
- `mempool_txs`: `HashMap` representando las transacciones actuales en la mempool.
- Elimina transacciones no presentes en `mempool_txs`.

## Uso de TxGraph en el Contexto del Script

`TxGraph` se utiliza para representar y manejar dinámicamente las relaciones entre transacciones en la mempool de Bitcoin. El script actualiza periódicamente esta estructura para reflejar el estado actual de la mempool.



## Descripción General del Programa

El script en Rust utiliza Rocket para crear una aplicación web, mostrando visualmente las relaciones entre transacciones en la mempool de Bitcoin.

### Características Principales

- **Configuración de Rocket y Estructuras**: Utiliza Rocket para la aplicación web y define `TxGraph` para almacenar relaciones entre transacciones.
- **Manejo de Conexiones a Bitcoin Core**: Establece conexión con un nodo de Bitcoin Core y maneja la autenticación.
- **Concurrencia y Actualización del Grafo**: Usa un hilo para actualizar el grafo de transacciones cada `SLEEP_TIME` segundos.
- **Obtención y Procesamiento de Transacciones**: Obtiene transacciones de la mempool y procesa sus relaciones.
- **Generación de Contenido HTML**: Ofrece una vista HTML de las transacciones y sus descendientes.

### Función `get_mempool_descendants`

- Obtiene los descendientes de una transacción específica en la mempool.



## Explicación de `get_descendants`

Esta función actualiza `TxGraph` con las relaciones entre transacciones en la mempool de Bitcoin, utilizando la API de Bitcoin Core.

### Parámetros de la Función

- `mempool_txs`: `HashMap<String, Value>` con las transacciones actuales.
- `client`: Referencia a un cliente de la API de Bitcoin Core.
- `graph`: Referencia compartida (`Arc<Mutex<TxGraph>>`) al grafo de transacciones.

### Funcionamiento de la Función

- **Obtención del Bloqueo del Grafo**: Accede exclusivamente al grafo para su actualización.
- **Iteración sobre las Transacciones en la Mempool**: Procesa cada transacción verificando y añadiendo descendientes.
- **Limitación de Procesamiento**: Controla el número de transacciones procesadas para evitar sobrecargas.

### Consideraciones Adicionales

- Uso de `Mutex` para la gestión segura del acceso concurrente al grafo.
- La función refleja cuidadosamente las relaciones entre transacciones en la mempool.


## Función `get_descen_json`

### Descripción
La función `get_descen_json` es una ruta del servidor web implementada en nuestro proyecto usando Rocket, un marco de trabajo web en Rust. Está diseñada para generar una respuesta en formato JSON que detalla las relaciones entre transacciones en la mempool de Bitcoin.

### Detalles Técnicos
- **Ruta**: `@get("/get_descen_json")`
- **Parámetros**: 
  - `graph`: Un estado compartido del tipo `Arc<Mutex<TxGraph>>`, representando un grafo de transacciones. Cada nodo en este grafo es una transacción en la mempool de Bitcoin.

### Funcionalidad
La función bloquea y desbloquea el estado compartido del grafo para acceder a su contenido actual. Itera sobre cada transacción (considerada como 'transacción padre') y sus transacciones asociadas ('transacciones hijas'). En caso de que las transacciones hijas tengan sus propias transacciones asociadas ('transacciones nietas'), también las incluye.

Para cada conjunto de relaciones padre-hijo-nieto, crea un objeto JSON estructurado. Este objeto refleja la jerarquía y las conexiones entre las transacciones. Al final, todas estas estructuras JSON se agrupan en un arreglo, que se serializa y se envía como respuesta al cliente.

### Respuesta
La respuesta es un objeto JSON que contiene un arreglo de transacciones, cada una con su identificador y los identificadores de sus transacciones descendientes (hijas y nietas), si las hay. Esta estructura proporciona una visión clara de la cadena de transacciones en la mempool de Bitcoin.

### Uso
Esta ruta es accesible mediante una petición GET al endpoint `/get_descen_json` del servidor. La respuesta puede ser utilizada para análisis de la mempool o visualización de la cadena de transacciones en aplicaciones cliente.

