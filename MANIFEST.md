📜 B4N1-MANIFEST: B4N1-WEB v0.9.0 (Agentic Browser)

    INSTRUCCIÓN SOBERANA: Este módulo es un navegador agéntico. Navega por la web de forma autónoma, ejecuta acciones en páginas web, extrae datos y completa flujos complejos usando IA.

1. IDENTIDAD Y SOBERANÍA (Identity)

    Misión: Navegar la web autónomamente para ejecutar tareas: extraer datos, llenar formularios, monitorear cambios y orquestar flujos multi-página.

    Alineación OMNI: Agente de ejecución para B4N1-INTEL (recolección), B4N1-FETCHER (descargas), B4N1-COMMERCE (monitoreo de precios).

    Motor: Headless browser engine con control vía IA (B4N1-IA-PROVIDER) para decidir qué acciones tomar en cada página.

2. ESQUEMA DE DATOS Y CAPACIDADES
Entidad	Campo	Tipo	CRUD	Notas de Lógica
BrowseSession	target_url	String	C-R	URL objetivo de la sesión.
BrowseSession	goal	String	C-R	Objetivo de la navegación.
PageAction	type	Enum	C	[Click, Type, Extract, Wait, Navigate, Screenshot].
PageAction	selector	String	C	Selector CSS/XPath del elemento.
PageAction	result	String	R	Resultado de la acción ejecutada.
ExtractedData	schema	JSON	C-R	Esquema de datos a extraer.
ExtractedData	values	JSON	R	Datos extraídos estructurados.

    Acciones Especiales:

        navigate: Navega a una URL y espera carga completa.

        extract: Extrae datos de la página según esquema definido.

        fill_form: Completa campos de formulario con datos proporcionados.

        monitor: Monitorea cambios en una página y alerta cuando hay diferencias.

        screenshot: Captura pantalla de la página en estado actual.

3. MATRIZ DE ACCESO
Rol	Capacidad	Visibilidad	Condición
User	Crear sesiones	Sus sesiones	Solo sus navegaciones
Admin	Full Control	Global	Puede ver todas las sesiones

4. TAXONOMÍA DE ERRORES
Código	UI Label	Causa	Acción
0xW01	"Elemento No Encontrado"	Selector no existe en DOM	Reintentar con selector alternativo
0xW02	"Página No Cargó"	Timeout de carga de página	Aumentar timeout o reintentar
0xW03	"CAPTCHA Detectado"	Página bloqueó el acceso	Pausar y notificar al usuario

5. MODELO DE NEGOCIO (Monetización)

    Plan FREE: 10 sesiones/mes, 5 acciones por sesión.

    Plan PRO ($19/mes): 100 sesiones/mes, extracción estructurada, monitoreo de cambios.

    Plan ENTERPRISE: Sesiones ilimitadas, proxies rotativos, bypass de bloqueos avanzados.

6. DEFINITION OF DONE (DoD)

    WEB navega a una URL, extrae datos estructurados y los devuelve en JSON.

    WEB puede hacer login en un sitio, navegar y extraer datos protegidos.

    El monitoreo de cambios detecta diferencias y alerta en < 5 minutos.

    Las sesiones fallidas reintentan con estrategias alternativas de navegación.

7. INTERFACES DE SALIDA (Output Interfaces)

    │ Tipo       │ Estado    │ Puerto / Binario │ Notas                         │
    │ b4n1-api   │ ⏳ futuro  │ 8410             │ REST API para control de sesión│
    │ b4n1-mcp   │ ✅ activa │ stdio            │ Model Context Protocol        │
    │ b4n1-cli   │ ✅ activa │ b4n1web          │ subcomandos: goto, get_links  │
    │ b4n1-tui   │ ⏳ futuro  │ —                │ —                             │
    │ b4n1-front │ ⏳ futuro  │ —                │ —                             │
