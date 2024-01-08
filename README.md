# Visualizador de la Mempool de Bitcoin

## Descripción General

Este programa en Rust proporciona una solución integral para visualizar y analizar las relaciones entre las transacciones en la mempool de la blockchain de Bitcoin. Utilizando el framework Rocket para el desarrollo web y la API de `bitcoincore_rpc` para la interacción con un nodo de Bitcoin Core, este script ofrece una manera eficiente y dinámica de entender las complejidades de las transacciones de Bitcoin en tiempo real.

## Características Principales

- **Interfaz Web con Rocket**: Implementa una aplicación web para visualizar las transacciones de la mempool de Bitcoin.
- **Conexión con Nodo de Bitcoin Core**: Se conecta a un nodo de Bitcoin Core para obtener datos en tiempo real.
- **Visualización de Relaciones de Transacciones**: Muestra las relaciones padre-hijo entre las transacciones en la mempool.
- **Actualización Dinámica del Grafo de Transacciones**: Utiliza una estructura de datos `TxGraph` para mantener y actualizar las relaciones entre transacciones.
- **Presentación en HTML**: Genera contenido HTML para visualizar las transacciones y sus relaciones de una manera clara y comprensible.
- **Manejo de Concurrencia**: Emplea hilos para gestionar la actualización periódica de la información y mantener la aplicación web responsiva.

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



