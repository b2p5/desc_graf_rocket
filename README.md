# Visualizador de la Mempool de Bitcoin

## Descripción General

Este programa en Rust proporciona una solución integral para visualizar y analizar las relaciones entre las transacciones en la mempool de la blockchain de Bitcoin. Utilizando el framework Rocket para el desarrollo web y la API de `bitcoincore_rpc` para la interacción con un nodo de Bitcoin Core, este script ofrece una manera eficiente y dinámica de entender las complejidades de las transacciones de Bitcoin en tiempo real.

## Características Principales

- **Interfaz Web con Rocket**: Implementa una aplicación web para visualizar las transacciones de la mempool de Bitcoin.
- **Conexión con Nodo de Bitcoin Core**: Se conecta a un nodo de Bitcoin Core para obtener datos en tiempo real.
- **Visualización de Relaciones de Transacciones**: Muestra las relaciones padre-hijo entre las transacciones en la mempool.
- **Actualización Dinámica del Grafo de Transacciones**: Utiliza una estructura de datos `TxGraph` para mantener y actualizar las relaciones entre transacciones.
- **Presentación en HTML**: Genera contenido HTML para visualizar las transacciones y sus relaciones de una manera clara y comprensible.
- **Manejo de Concurrency**: Emplea hilos para gestionar la actualización periódica de la información y mantener la aplicación web responsiva.

## Estructura y Funcionamiento

### TxGraph: Estructura de Datos Central

- **Gestión de Relaciones de Transacciones**: Almacena y actualiza las relaciones entre transacciones en la mempool.
- **Métodos para la Manipulación del Grafo**: Incluye funciones para añadir y limpiar transacciones, manteniendo el grafo actualizado con el estado actual de la mempool.

### Interacción con el Nodo Bitcoin Core

- **Recuperación de Datos de la Mempool**: Utiliza `bitcoincore_rpc` para consultar el nodo de Bitcoin Core y obtener transacciones de la mempool.

### Servidor Web Rocket

- **Endpoint `/get_descen_html`**: Proporciona una interfaz web para visualizar las transacciones y sus descendientes.
- **Generación Dinámica de Contenido HTML**: Crea vistas HTML que representan las transacciones y sus relaciones en el rango especificado.

### Funciones Clave

- **`get_raw_mempool`**: Obtiene las transacciones actuales de la mempool.
- **`get_descendants`**: Procesa las transacciones para identificar y añadir relaciones padre-hijo al grafo.
- **`get_mempool_descendants`**: Obtiene los descendientes de una transacción específica en la mempool.

## Requisitos

- Rust y Cargo (última versión estable)
- Acceso a un nodo de Bitcoin Core en funcionamiento
- Sistema operativo Unix/Linux preferentemente

## Instalación y Ejecución

1. Clonar el repositorio.
2. Navegar al directorio del proyecto y ejecutar `cargo build --release`.
3. Iniciar el programa con `./target/release/[nombre_del_programa]`.

## Contribuciones y Soporte

Las contribuciones son bienvenidas. Para problemas, preguntas o contribuciones, por favor abre un issue o un pull request en el repositorio.

## Licencia

Este proyecto está bajo [INSERTE LICENCIA AQUÍ], vea el archivo LICENSE para más detalles.





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
