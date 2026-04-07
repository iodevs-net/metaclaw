#!/bin/bash
# Setup interactivo para MCPs de cloud storage
# Uso: ./scripts/setup-cloud-mcp.sh

set -e

CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${CYAN}"
echo "╔════════════════════════════════════════════════════╗"
echo "║   MCP Cloud Storage Setup - Metaclaw              ║"
echo "╚════════════════════════════════════════════════════╝"
echo -e "${NC}"

show_menu() {
    echo ""
    echo "¿Qué servicio quieres configurar?"
    echo ""
    echo "  1) Google Drive"
    echo "  2) Dropbox"
    echo "  3) OneDrive / Microsoft 365"
    echo "  4) Todos"
    echo "  5) Ver estado actual"
    echo "  0) Salir"
    echo ""
}

show_status() {
    echo -e "${CYAN}Estado actual de credenciales:${NC}"
    echo ""
    
    if [ -n "$GOOGLE_CLIENT_ID" ]; then
        echo -e "  ${GREEN}✓${NC} Google Drive: configurado"
    else
        echo -e "  ${RED}✗${NC} Google Drive: FALTA"
    fi
    
    if [ -n "$DROPBOX_ACCESS_TOKEN" ]; then
        echo -e "  ${GREEN}✓${NC} Dropbox: configurado"
    else
        echo -e "  ${RED}✗${NC} Dropbox: FALTA"
    fi
    
    if [ -n "$MICROSOFT_CLIENT_ID" ]; then
        echo -e "  ${GREEN}✓${NC} OneDrive: configurado"
    else
        echo -e "  ${RED}✗${NC} OneDrive: FALTA"
    fi
    
    echo ""
}

configure_google_drive() {
    echo -e "${CYAN}=== Google Drive ===${NC}"
    echo ""
    echo "Sigue la guía: docs/getting-started/CREDENTIALS_GOOGLE_DRIVE.md"
    echo ""
    read -p "Ingresa GOOGLE_CLIENT_ID: " val && [ -n "$val" ] && sed -i "s/^GOOGLE_CLIENT_ID=.*/GOOGLE_CLIENT_ID=$val/" .env
    read -p "Ingresa GOOGLE_CLIENT_SECRET: " val && [ -n "$val" ] && sed -i "s/^GOOGLE_CLIENT_SECRET=.*/GOOGLE_CLIENT_SECRET=$val/" .env
    read -p "Ingresa GOOGLE_REFRESH_TOKEN: " val && [ -n "$val" ] && sed -i "s/^GOOGLE_REFRESH_TOKEN=.*/GOOGLE_REFRESH_TOKEN=$val/" .env
    echo -e "${GREEN}✓ Google Drive configurado${NC}"
}

configure_dropbox() {
    echo -e "${CYAN}=== Dropbox ===${NC}"
    echo ""
    echo "Sigue la guía: docs/getting-started/CREDENTIALS_DROPBOX.md"
    echo ""
    read -p "Ingresa DROPBOX_ACCESS_TOKEN: " val && [ -n "$val" ] && sed -i "s/^DROPBOX_ACCESS_TOKEN=.*/DROPBOX_ACCESS_TOKEN=$val/" .env
    echo -e "${GREEN}✓ Dropbox configurado${NC}"
}

configure_onedrive() {
    echo -e "${CYAN}=== OneDrive / Microsoft 365 ===${NC}"
    echo ""
    echo "Sigue la guía: docs/getting-started/CREDENTIALS_ONEDRIVE.md"
    echo ""
    read -p "Ingresa MICROSOFT_CLIENT_ID: " val && [ -n "$val" ] && sed -i "s/^MICROSOFT_CLIENT_ID=.*/MICROSOFT_CLIENT_ID=$val/" .env
    read -p "Ingresa MICROSOFT_TENANT_ID (ENTER para 'common'): " val
    [ -z "$val" ] && val="common"
    sed -i "s/^MICROSOFT_TENANT_ID=.*/MICROSOFT_TENANT_ID=$val/" .env
    echo -e "${GREEN}✓ OneDrive configurado${NC}"
}

activate_mcp() {
    echo -e "${CYAN}Activando MCPs...${NC}"
    source .env
    ./scripts/activate-mcp-cloud.sh
}

restart_container() {
    echo -e "${CYAN}Reiniciando metaclaw...${NC}"
    docker compose -f docker-compose.dev.yml restart
    sleep 3
    echo -e "${GREEN}✓ Reiniciado${NC}"
}

# Cargar .env si existe
[ -f .env ] && source .env

while true; do
    show_menu
    read -p "Opción: " opt
    case $opt in
        1) configure_google_drive ;;
        2) configure_dropbox ;;
        3) configure_onedrive ;;
        4) configure_google_drive; configure_dropbox; configure_onedrive ;;
        5) show_status ;;
        0) echo "Bye!"; exit 0 ;;
        *) echo "Opción inválida" ;;
    esac
    
    # Recargar después de cambios
    [ -f .env ] && source .env
    
    # Preguntar si activar
    if [ -n "$GOOGLE_CLIENT_ID" ] || [ -n "$DROPBOX_ACCESS_TOKEN" ] || [ -n "$MICROSOFT_CLIENT_ID" ]; then
        echo ""
        read -p "¿Activar MCPs y reiniciar? (s/n): " confirm
        [ "$confirm" = "s" ] && activate_mcp && restart_container
    fi
done
