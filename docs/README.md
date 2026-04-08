# MetaClaw - Centro de Documentación

> **Acerca de MetaClaw**
>
> MetaClaw es un fork personal de [ZeroClaw](https://github.com/zeroclaw-labs/zeroclaw), mantenido localmente para uso personal/interno.
>
> - **Upstream**: Fork de ZeroClaw (repositorio local en `/home/leonardo/dev/proyectos/zeroclaw`)
> - **Propósito**: Uso personal con modificaciones específicas
> - **Compatibilidad**: Mantiene compatibilidad con la API y configuración de ZeroClaw
>
> Para información general sobre ZeroClaw, consulta el [repositorio upstream](https://github.com/zeroclaw-labs/zeroclaw).

Esta página es el punto de entrada principal para el sistema de documentación.

## Empezar Aquí

| Quiero...                                     | Leer esto                                                           |
| --------------------------------------------- | ------------------------------------------------------------------- |
| Instalar y ejecutar MetaClaw rápidamente      | [README.md (Inicio Rápido)](../README.md#quick-start)               |
| Bootstrap en un comando                       | [one-click-bootstrap.md](setup-guides/one-click-bootstrap.md)       |
| Actualizar o desinstalar en macOS             | [macos-update-uninstall.md](setup-guides/macos-update-uninstall.md) |
| Encontrar comandos por tarea                  | [commands-reference.md](reference/cli/commands-reference.md)        |
| Ver configuración de claves rápidamente       | [config-reference.md](reference/api/config-reference.md)            |
| Configurar providers/endpoints personalizados | [custom-providers.md](contributing/custom-providers.md)             |
| Configurar proveedor Z.AI / GLM               | [zai-glm-setup.md](setup-guides/zai-glm-setup.md)                   |
| Usar patrones de integración LangGraph        | [langgraph-integration.md](contributing/langgraph-integration.md)   |
| Operar runtime (runbook día 2)                | [operations-runbook.md](ops/operations-runbook.md)                  |
| Solucionar problemas de instalación/runtime   | [troubleshooting.md](ops/troubleshooting.md)                        |
| Configurar Matrix con E2EE y diagnósticos     | [matrix-e2ee-guide.md](security/matrix-e2ee-guide.md)               |
| Navegar docs por categoría                    | [SUMMARY.md](SUMMARY.md)                                            |

## Árbol de Decisión Rápido (10 segundos)

- ¿Primera instalación? → [setup-guides/README.md](setup-guides/README.md)
- ¿Necesitas CLI/config exactos? → [reference/README.md](reference/README.md)
- ¿Operaciones de producción/servicio? → [ops/README.md](ops/README.md)
- ¿Fallos o regresiones? → [troubleshooting.md](ops/troubleshooting.md)
- ¿Seguridad o roadmap? → [security/README.md](security/README.md)
- ¿Placas/periféricos? → [hardware/README.md](hardware/README.md)
- ¿Contribución/revisión/CI? → [contributing/README.md](contributing/README.md)
- ¿Mapa completo? → [SUMMARY.md](SUMMARY.md)

## Colecciones (Recomendadas)

- Primeros pasos: [setup-guides/README.md](setup-guides/README.md)
- Catálogos de referencia: [reference/README.md](reference/README.md)
- Operaciones y despliegue: [ops/README.md](ops/README.md)
- Documentos de seguridad: [security/README.md](security/README.md)
- Hardware/periféricos: [hardware/README.md](hardware/README.md)
- Contribución/CI: [contributing/README.md](contributing/README.md)
- Snapshots del proyecto: [maintainers/README.md](maintainers/README.md)

## Por Audiencia

### Usuarios / Operadores

- [commands-reference.md](reference/cli/commands-reference.md) — búsqueda de comandos por workflow
- [providers-reference.md](reference/api/providers-reference.md) — IDs de providers, aliases, env vars
- [channels-reference.md](reference/api/channels-reference.md) — capacidades y rutas de setup
- [matrix-e2ee-guide.md](security/matrix-e2ee-guide.md) — setup de Matrix E2EE y diagnósticos
- [config-reference.md](reference/api/config-reference.md) — claves de configuración y defaults seguros
- [custom-providers.md](contributing/custom-providers.md) — templates de integración personalizados
- [zai-glm-setup.md](setup-guides/zai-glm-setup.md) — setup de Z.AI/GLM
- [langgraph-integration.md](contributing/langgraph-integration.md) — integración con LangGraph
- [operations-runbook.md](ops/operations-runbook.md) — operaciones runtime y rollback
- [troubleshooting.md](ops/troubleshooting.md) — firmas de fallo comunes y recuperación

### Contribuidores / Mantenedores

- [../CONTRIBUTING.md](../CONTRIBUTING.md)
- [pr-workflow.md](contributing/pr-workflow.md)
- [reviewer-playbook.md](contributing/reviewer-playbook.md)
- [ci-map.md](contributing/ci-map.md)
- [actions-source-policy.md](contributing/actions-source-policy.md)

### Seguridad / Confiabilidad

> Nota: esta área incluye documentos de propuesta/roadmap. Para comportamiento actual, comenzar con [config-reference.md](reference/api/config-reference.md), [operations-runbook.md](ops/operations-runbook.md), y [troubleshooting.md](ops/troubleshooting.md).

- [security/README.md](security/README.md)
- [agnostic-security.md](security/agnostic-security.md)
- [frictionless-security.md](security/frictionless-security.md)
- [sandboxing.md](security/sandboxing.md)
- [audit-logging.md](security/audit-logging.md)
- [resource-limits.md](ops/resource-limits.md)
- [security-roadmap.md](security/security-roadmap.md)

## Navegación del Sistema

- Tabla de contenidos unificada: [SUMMARY.md](SUMMARY.md)
- Índice de docs i18n: [i18n/README.md](i18n/README.md)
